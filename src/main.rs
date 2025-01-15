mod graph;
mod standard_algorithms;
mod fineman_algorithm;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    for _ in 0..100000 {
        //
    }
    let graph = graph::Graph::new(10);
    let proper_graph = graph::ProperGraph::try_from(graph).unwrap();
}
