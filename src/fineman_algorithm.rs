mod framework;

mod betweenness_reduction;

mod finding_sandwich;

mod sandwiches_to_r_remoteness;

mod hop_reduction;

use crate::graph::*;
use crate::standard_algorithms::*;

#[derive(Debug, Clone)]
pub struct PricedGraph {
    pub n: usize,
    pub pedges: Vec<Vec<(usize, i64)>>,
    pub pedges_rev: Vec<Vec<(usize, i64)>>,
    pub nedges: Vec<Vec<(usize, i64, bool)>>,
    pub nedges_rev: Vec<Vec<(usize, i64, bool)>>,
    pub prices: Vec<i64>,
}

impl PricedGraph {
    pub fn as_borrowed(&self) -> BorrowedWeightedGraph<'_> {
        BorrowedWeightedGraph {
            n: self.n,
            pedges: &self.pedges,
            nedges: &self.nedges,
        }
    }

    pub fn as_borrowed_rev(&self) -> BorrowedWeightedGraph<'_> {
        BorrowedWeightedGraph {
            n: self.n,
            pedges: &self.pedges_rev,
            nedges: &self.nedges_rev,
        }
    }

    pub fn sanity_check(&self) {
        // if cfg!(not(debug_assertions)) {
        //     return;
        // }
        let n = self.n;
        assert_eq!(self.pedges.len(), n);
        assert_eq!(self.pedges_rev.len(), n);
        assert_eq!(self.nedges.len(), n);
        assert_eq!(self.nedges_rev.len(), n);
        assert_eq!(self.prices.len(), n);
        assert!(self
            .pedges
            .iter()
            .all(|edges_u| edges_u.iter().all(|&(v, w)| v < n && w >= 0)));
        assert!(self
            .pedges_rev
            .iter()
            .all(|edges_u| edges_u.iter().all(|&(v, w)| v < n && w >= 0)));
        assert!(self
            .nedges
            .iter()
            .all(|edges_u| edges_u.iter().all(|&(v, _, _)| v < n)));
        assert!(self
            .nedges_rev
            .iter()
            .all(|edges_u| edges_u.iter().all(|&(v, _, _)| v < n)));
    }

    pub fn check_eliminated(&self) {
        // if cfg!(not(debug_assertions)) {
        //     return;
        // }
        self.sanity_check();
        assert!(self.nedges.iter().map(|edges_u| edges_u.len()).sum::<usize>() == 0);
    }

    pub fn apply_price(&mut self, p: &[i64]) {
        for (u, edges_u) in self.pedges.iter_mut().enumerate() {
            for (v, w) in edges_u.iter_mut() {
                *w += p[u] - p[*v];
            }
        }
        for (u, edges_u) in self.nedges.iter_mut().enumerate() {
            for (v, w, _) in edges_u.iter_mut() {
                *w += p[u] - p[*v];
            }
        }
        for (u, edges_u) in self.pedges_rev.iter_mut().enumerate() {
            for (v, w) in edges_u.iter_mut() {
                *w += p[*v] - p[u];
            }
        }
        for (u, edges_u) in self.nedges_rev.iter_mut().enumerate() {
            for (v, w, _) in edges_u.iter_mut() {
                *w += p[*v] - p[u];
            }
        }
        for (u, price_u) in self.prices.iter_mut().enumerate() {
            *price_u += p[u];
        }
        self.sanity_check();
    }

    pub fn unapply_price(&mut self, p: &[i64]) {
        let p_neg: Vec<_> = p.iter().map(|&x| -x).collect();
        self.apply_price(&p_neg);
        self.sanity_check();
    }
}

impl From<ProperGraph> for PricedGraph {
    fn from(graph: ProperGraph) -> Self {
        let n = graph.n;
        let mut pedges = vec![vec![]; n];
        let mut pedges_rev = vec![vec![]; n];
        let mut nedges = vec![vec![]; n];
        let mut nedges_rev = vec![vec![]; n];
        for u in 0..n {
            for &(v, w) in &graph.edges[u] {
                if w >= 0 {
                    pedges[u].push((v, w));
                    pedges_rev[v].push((u, w));
                } else {
                    nedges[u].push((v, w, true));
                    nedges_rev[v].push((u, w, true));
                }
            }
        }
        let prices = vec![0; n];
        let priced_graph = PricedGraph {
            n,
            pedges,
            pedges_rev,
            nedges,
            nedges_rev,
            prices,
        };
        priced_graph.sanity_check();
        priced_graph
    }
}

fn bellman_ford_dijkstra_up_to_h_hops(
    graph: BorrowedWeightedGraph<'_>,
    h: usize,
    mut dist: Vec<Option<i64>>,
) -> Vec<Option<i64>> {
    dist = dijkstra(graph, dist);
    for _ in 0..h {
        let mut ndist = dist.clone();
        for (u, &dist_u) in dist.iter().enumerate() {
            if let Some(d) = dist_u {
                for &(v, w, t) in &graph.nedges[u] {
                    if t && ndist[v].map_or(true, |d2| d + w < d2) {
                        ndist[v] = Some(d + w);
                    }
                }
            }
        }
        dist = dijkstra(graph, ndist);
    }
    dist
}

fn bellman_ford_dijkstra_up_to_h_hops_with_origins(
    graph: BorrowedWeightedGraph<'_>,
    h: usize,
    mut dist: Vec<Option<(i64, usize)>>,
) -> Vec<Option<(i64, usize)>> {
    dist = dijkstra_with_origins(graph, dist);
    for _ in 0..h {
        let mut ndist = dist.clone();
        for (u, &dist_u) in dist.iter().enumerate() {
            if let Some((d, o)) = dist_u {
                for &(v, w, t) in &graph.nedges[u] {
                    if t && ndist[v].map_or(true, |(d2, _)| d + w < d2) {
                        ndist[v] = Some((d + w, o));
                    }
                }
            }
        }
        dist = dijkstra_with_origins(graph, ndist);
    }
    dist
}

fn bellman_ford_dijkstra_with_hops_bound(
    graph: BorrowedWeightedGraph<'_>,
    h: usize,
) -> Result<Vec<i64>, ()> {
    let mut dist = dijkstra(graph, vec![Some(0); graph.n]);
    for _ in 0..h {
        let mut relaxed = false;
        let mut ndist = dist.clone();
        for (u, &dist_u) in dist.iter().enumerate() {
            if let Some(d) = dist_u {
                for &(v, w, t) in &graph.nedges[u] {
                    if t && ndist[v].unwrap() < d + w {
                        ndist[v] = Some(d + w);
                        relaxed = true;
                    }
                }
            }
        }
        if !relaxed {
            return Ok(dist.into_iter().map(|d| d.unwrap()).collect());
        }
        dist = dijkstra(graph, ndist);
    }
    for (u, &dist_u) in dist.iter().enumerate() {
        if let Some(d) = dist_u {
            for &(v, w, t) in &graph.nedges[u] {
                if t && dist[v].unwrap() < d + w {
                    return Err(());
                }
            }
        }
    }
    Ok(dist.into_iter().map(|d| d.unwrap()).collect())
}

fn negative_h_hop_reach(graph: BorrowedWeightedGraph<'_>, h: usize, start: &[usize]) -> Vec<usize> {
    let mut dist = vec![None; graph.n];
    for &u in start {
        dist[u] = Some(0);
    }
    dist = bellman_ford_dijkstra_up_to_h_hops(graph, h, dist);
    dist.iter()
        .enumerate()
        .filter_map(|(u, &d)| {
            if d.is_some() && d.unwrap() < 0 {
                Some(u)
            } else {
                None
            }
        })
        .collect()
}
