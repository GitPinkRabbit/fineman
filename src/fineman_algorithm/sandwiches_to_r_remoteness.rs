use super::*;

pub fn sandwiches_to_r_remoteness(
    graph: &mut PricedGraph,
    x: usize,
    u: &[usize],
    y: usize,
    beta: usize,
) -> Vec<i64> {
    let n = graph.n;
    for u in 0..n {
        for (_, _, t) in &mut graph.nedges[u] {
            *t = true;
        }
        for (_, _, t) in &mut graph.nedges_rev[u] {
            *t = true;
        }
    }
    let mut dis_from_x = vec![None; n];
    dis_from_x[x] = Some(0);
    let dis_from_x = bellman_ford_dijkstra_up_to_h_hops(graph.as_borrowed(), beta, dis_from_x);
    let mut dis_to_y = vec![None; n];
    dis_to_y[y] = Some(0);
    let dis_to_y = bellman_ford_dijkstra_up_to_h_hops(graph.as_borrowed_rev(), beta, dis_to_y);
    let p: Vec<_> = (0..n).map(|u| dis_from_x[u].map_or(0, |dx| dis_to_y[u].map_or(0, |dy| 0.min(dx.max(-dy))))).collect();
    graph.apply_price(&p);
    p
}
