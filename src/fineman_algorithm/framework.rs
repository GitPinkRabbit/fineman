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
    unimplemented!()
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
