use super::*;

use rand::prelude::*;

pub fn betweenness_reduction(
    graph: &mut PricedGraph,
    beta: usize,
    tau: usize,
) -> Result<Vec<i64>, ()> {
    let n = graph.n;
    for u in 0..n {
        for (_, _, t) in &mut graph.nedges[u] {
            *t = true;
        }
        for (_, _, t) in &mut graph.nedges_rev[u] {
            *t = true;
        }
    }
    let cc = 4;
    let t = ((((cc * tau) as f64 * (n as f64).ln()).ceil() + 0.5) as usize).clamp(1, n);
    // randomly sample t vertices from 0..n
    let mut rng = thread_rng();
    let mut vertices = (0..n).collect::<Vec<_>>();
    let ts = vertices.partial_shuffle(&mut rng, t).0.to_vec();
    unimplemented!()
}
