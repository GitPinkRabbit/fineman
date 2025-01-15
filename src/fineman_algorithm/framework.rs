use super::*;

pub fn sssp(graph: ProperGraph, source: usize) -> Result<Vec<Option<i64>>, ()> {
    let mut graph: PricedGraph = graph.into();
    final_price(&mut graph)?;
    graph.check_eliminated();
    let p = &graph.prices[..];
    let mut dist = vec![None; graph.n];
    dist[source] = Some(0);
    dist = dijkstra(graph.as_borrowed(), dist);
    Ok(dist
        .iter()
        .enumerate()
        .map(|(u, &d)| d.map(|d| d - p[source] + p[u]))
        .collect())
}

fn final_price(graph: &mut PricedGraph) -> Result<(), ()> {
    while graph.nedges.iter().any(|edges| !edges.is_empty()) {
        eliminate_once(graph)?;
        rebuild(graph);
    }
    Ok(())
}

fn eliminate_once(graph: &mut PricedGraph) -> Result<(), ()> {
    let k = graph.nedges.iter().map(|edges| edges.len()).sum::<usize>();
    assert!(k >= 1);
    let r = (((k as f64).powf(1. / 9.).ceil() + 0.5) as usize).clamp(1, k);
    let r_remote = loop {
        let p1 = betweenness_reduction::betweenness_reduction(graph, r + 1, r)?;
        let result = finding_sandwich::finding_sandwich(graph)?;
        let s = match result {
            finding_sandwich::Sandwich(s) => s,
            finding_sandwich::IndependentSet(i) => {
                eliminate_independent_set(graph, &i);
                return Ok(());
            }
        };
        let (x, u, y) = s;
        let p2 = sandwiches_to_r_remoteness::sandwiches_to_r_remoteness(graph, x, &u, y, r + 1);
        let size = negative_h_hop_reach(graph.as_borrowed(), r, &u).len();
        if size * r > graph.n {
            graph.unapply_price(&p2);
            graph.unapply_price(&p1);
            continue;
        }
        break u;
    };
    hop_reduction::hop_reduction(graph, &r_remote, r)?;
    Ok(())
}

fn eliminate_independent_set(graph: &mut PricedGraph, i: &[usize]) {
    let mut b = vec![0; graph.n];
    for &u in i {
        b[u] = 1;
    }
    let b = b;
    for (u, ne) in graph.nedges.iter_mut().enumerate() {
        for (_, _, t) in ne.iter_mut() {
            *t = b[u] == 1;
        }
    }
    let p = bellman_ford_dijkstra_with_hops_bound(graph.as_borrowed(), 1).unwrap();
    graph.apply_price(&p);
}

fn rebuild(graph: &mut PricedGraph) {
    let mut cnt = 0;
    for (u, nedges_u) in graph.nedges.iter_mut().enumerate() {
        let (pe, ne): (Vec<_>, Vec<_>) = nedges_u
            .iter()
            .cloned()
            .map(|(v, w, _)| (v, w, true))
            .partition(|&(_, w, _)| w >= 0);
        *nedges_u = ne;
        cnt += pe.len();
        for (v, w, _) in pe {
            graph.pedges[u].push((v, w));
            graph.pedges_rev[v].push((u, w));
        }
        graph.nedges_rev[u] = graph.nedges_rev[u]
            .iter()
            .cloned()
            .map(|(v, w, _)| (v, w, true))
            .filter(|&(_, w, _)| w < 0)
            .collect();
    }
    assert!(cnt >= 1);
    graph.sanity_check();
}
