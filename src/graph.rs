#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize,
    pub edges: Vec<Vec<(usize, i64)>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            edges: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        // check bounds
        assert!(u < self.n && v < self.n);
        self.edges[u].push((v, w));
    }
}

#[derive(Debug, Clone)]
struct SimpleGraph {
    pub n: usize,
    pub edges: Vec<Vec<(usize, i64)>>,
}

impl SimpleGraph {
    pub fn sanity_check(&self) {
        // if cfg!(not(debug_assertions)) {
        //     return;
        // }
        let n = self.n;
        let edges = &self.edges;
        assert!(edges.len() == n);
        assert!(edges.iter().all(|v| v.iter().all(|&(u, _)| u < n)));
        let mut seen = vec![false; n];
        for u in 0..n {
            for &(v, _) in &edges[u] {
                assert!(!seen[v]);
                seen[v] = true;
            }
            assert!(!seen[u]);
            for &(v, _) in &edges[u] {
                seen[v] = false;
            }
        }
    }
}

impl TryFrom<Graph> for SimpleGraph {
    type Error = ();

    fn try_from(graph: Graph) -> Result<Self, Self::Error> {
        let n = graph.n;
        let mut edges = vec![Vec::new(); n];
        for (u, edges_u) in graph.edges.iter().enumerate() {
            // remove self loops and duplicate edges by sorting the edges
            let mut edges_u = edges_u.clone();
            edges_u.sort();
            for i in 0..edges_u.len() {
                if edges_u[i].0 == u {
                    if edges_u[i].1 < 0 {
                        return Err(());
                    }
                    continue;
                }
                if i > 0 && edges_u[i].0 == edges_u[i - 1].0 {
                    continue;
                }
                edges[u].push(edges_u[i]);
            }
        }
        let simple_graph = SimpleGraph { n, edges };
        simple_graph.sanity_check();
        Ok(simple_graph)
    }
}

#[derive(Debug, Clone)]
pub struct ProperGraph {
    pub n: usize,
    pub edges: Vec<Vec<(usize, i64)>>,
}

impl ProperGraph {
    pub fn sanity_check(&self) {
        // if cfg!(not(debug_assertions)) {
        //     return;
        // }
        let n = self.n;
        let edges = &self.edges;
        assert!(edges.len() == n);
        assert!(edges.iter().all(|v| v.iter().all(|&(u, _)| u < n)));
        let mut seen = vec![false; n];
        for u in 0..n {
            for &(v, _) in &edges[u] {
                assert!(!seen[v]);
                seen[v] = true;
            }
            assert!(!seen[u]);
            for &(v, _) in &edges[u] {
                seen[v] = false;
            }
        }
        let m: usize = edges.iter().map(|v| v.len()).sum();
        if m == 0 {
            return;
        }
        let degree_bound = (4 * m).div_ceil(n) + 1;
        let mut indeg = vec![0; n];
        for edges_u in edges {
            for &(v, _) in edges_u {
                indeg[v] += 1;
            }
        }
        for u in 0..n {
            assert!(indeg[u] <= degree_bound);
            assert!(edges[u].len() <= degree_bound);
        }
    }

    pub fn max_degree_vs_degree_bound(&self) -> (usize, usize) {
        let n = self.n;
        let edges = &self.edges;
        let m: usize = edges.iter().map(|v| v.len()).sum();
        if m == 0 {
            return (0, 1);
        }
        let degree_bound = (4 * m).div_ceil(n) + 1;
        let mut indeg = vec![0; n];
        for edges_u in edges {
            for &(v, _) in edges_u {
                indeg[v] += 1;
            }
        }
        let max_indeg = indeg.iter().max().copied().unwrap();
        let mex_outdeg = edges.iter().map(|v| v.len()).max().unwrap();
        (max_indeg.max(mex_outdeg), degree_bound)
    }
}

impl From<SimpleGraph> for ProperGraph {
    fn from(graph: SimpleGraph) -> Self {
        let n = graph.n;
        let m: usize = graph.edges.iter().map(|v| v.len()).sum();
        if m == 0 {
            return ProperGraph {
                n,
                edges: graph.edges,
            };
        }
        let degree_bound = (2 * m).div_ceil(n) + 1;
        debug_assert!(degree_bound >= 2);
        let outdeg = graph.edges.iter().map(|v| v.len()).collect::<Vec<_>>();
        let mut indeg = vec![0; n];
        for u in 0..n {
            for &(v, _) in &graph.edges[u] {
                indeg[v] += 1;
            }
        }
        let indeg = indeg;
        let mut cur_n = n;
        let mut outaux = vec![Vec::new(); n];
        for u in 0..n {
            outaux[u].push(u);
            let mut deg = outdeg[u];
            while deg > degree_bound {
                outaux[u].push(cur_n);
                cur_n += 1;
                deg -= degree_bound - 1;
            }
        }
        let mut inaux = vec![Vec::new(); n];
        for v in 0..n {
            inaux[v].push(v);
            let mut deg = indeg[v];
            while deg > degree_bound {
                inaux[v].push(cur_n);
                cur_n += 1;
                deg -= degree_bound - 1;
            }
        }
        let cur_n = cur_n;
        let mut outdeg = vec![0; cur_n];
        let mut indeg = vec![0; cur_n];
        let mut edges = vec![Vec::new(); cur_n];
        for u in 0..n {
            for i in 1..outaux[u].len() {
                edges[outaux[u][i - 1]].push((outaux[u][i], 0));
                outdeg[outaux[u][i - 1]] += 1;
                indeg[outaux[u][i]] += 1;
            }
            for i in 1..inaux[u].len() {
                edges[inaux[u][i]].push((inaux[u][i - 1], 0));
                outdeg[inaux[u][i]] += 1;
                indeg[inaux[u][i - 1]] += 1;
            }
        }
        for u in 0..n {
            for &(v, w) in &graph.edges[u] {
                if outdeg[*outaux[u].last().unwrap()] == degree_bound {
                    outaux[u].pop();
                }
                if indeg[*inaux[v].last().unwrap()] == degree_bound {
                    inaux[v].pop();
                }
                let uu = *outaux[u].last().unwrap();
                let vv = *inaux[v].last().unwrap();
                edges[uu].push((vv, w));
                outdeg[uu] += 1;
                indeg[vv] += 1;
            }
        }
        let proper_graph = ProperGraph { n: cur_n, edges };
        proper_graph.sanity_check();
        proper_graph
    }
}

impl TryFrom<Graph> for ProperGraph {
    type Error = ();

    fn try_from(graph: Graph) -> Result<Self, Self::Error> {
        let simple_graph: SimpleGraph = graph.try_into()?;
        Ok(ProperGraph::from(simple_graph))
    }
}
