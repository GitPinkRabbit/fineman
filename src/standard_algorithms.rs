use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone, Copy)]
pub struct BorrowedWeightedGraph<'a> {
    pub n: usize,
    pub pedges: &'a Vec<Vec<(usize, i64)>>,
    pub nedges: &'a Vec<Vec<(usize, i64, bool)>>,
}

pub fn dijkstra(graph: BorrowedWeightedGraph<'_>, mut dist: Vec<Option<i64>>) -> Vec<Option<i64>> {
    let mut heap = BinaryHeap::new();
    for (u, &dist_u) in dist.iter().enumerate() {
        if let Some(d) = dist_u {
            heap.push((Reverse(d), u));
        }
    }
    while let Some((Reverse(d), u)) = heap.pop() {
        if d > dist[u].unwrap() {
            continue;
        }
        for &(v, w) in &graph.pedges[u] {
            if dist[v].map_or(true, |d2| d + w < d2) {
                dist[v] = Some(d + w);
                heap.push((Reverse(d + w), v));
            }
        }
    }
    dist
}

pub fn dijkstra_with_origins(
    graph: BorrowedWeightedGraph<'_>,
    mut dist: Vec<Option<(i64, usize)>>,
) -> Vec<Option<(i64, usize)>> {
    let mut heap = BinaryHeap::new();
    for (u, &dist_u) in dist.iter().enumerate() {
        if let Some((d, o)) = dist_u {
            heap.push((Reverse(d), u, o));
        }
    }
    while let Some((Reverse(d), u, o)) = heap.pop() {
        if d > dist[u].unwrap().0 {
            continue;
        }
        for &(v, w) in &graph.pedges[u] {
            if dist[v].map_or(true, |(d2, _)| d + w < d2) {
                dist[v] = Some((d + w, o));
                heap.push((Reverse(d + w), v, o));
            }
        }
    }
    dist
}

pub fn bellman_ford(graph: BorrowedWeightedGraph<'_>) -> Result<Vec<i64>, ()> {
    let n = graph.n;
    let mut dist = vec![0; n];
    for _ in 0..n {
        let mut relaxed = false;
        for u in 0..n {
            for &(v, w) in &graph.pedges[u] {
                if dist[v] > dist[u] + w {
                    dist[v] = dist[u] + w;
                    relaxed = true;
                }
            }
            for &(v, w, t) in &graph.nedges[u] {
                if t && dist[v] > dist[u] + w {
                    dist[v] = dist[u] + w;
                    relaxed = true;
                }
            }
        }
        if !relaxed {
            return Ok(dist);
        }
    }
    Err(())
}
