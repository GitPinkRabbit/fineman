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
    unimplemented!()
}
