use super::*;

pub enum Outcome {
    Sandwich((usize, Vec<usize>, usize)),
    IndependentSet(Vec<usize>),
}
pub use Outcome::*;

pub fn finding_sandwich(graph: &mut PricedGraph) -> Result<Outcome, ()> {
    unimplemented!()
}
