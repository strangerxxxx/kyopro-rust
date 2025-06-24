use hld::HLDGraph;
#[allow(dead_code)]
mod hld {
    use std::cmp::Reverse;

    pub struct HLDGraph {
        parent: Vec<usize>,
        index: Vec<usize>,
        head: Vec<usize>,
        subtree_size: Vec<usize>,
    }

    impl HLDGraph {
        pub fn from_edges(
            root: usize,
            edges: impl ExactSizeIterator<Item = (usize, usize)>,
        ) -> Self {
            let size = edges.len() + 1;
            let mut graph = vec![vec![]; size];
            for (u, v) in edges {
                graph[u].push(v);
                graph[v].push(u);
            }

            let mut sorted = Vec::with_capacity(size);
            let mut parent = vec![!0; size];
            let mut stack = vec![root];
            while let Some(v) = stack.pop() {
                sorted.push(v);
                graph[v].retain(|&u| u != parent[v]);
                for &u in &graph[v] {
                    parent[u] = v;
                    stack.push(u);
                }
            }

            let mut subtree_size = vec![1; size];
            for &v in sorted.iter().rev() {
                if !graph[v].is_empty() {
                    graph[v].select_nth_unstable_by_key(0, |&c| Reverse(subtree_size[c]));
                }
                if v != root {
                    subtree_size[parent[v]] += subtree_size[v];
                }
            }

            let mut index = vec![!0; size];
            let mut head = (0..size).collect::<Vec<_>>();
            let mut current_index = 0;
            stack.push(root);
            while let Some(v) = stack.pop() {
                index[v] = current_index;
                current_index += 1;
                for (i, &c) in graph[v].iter().enumerate().rev() {
                    if i == 0 {
                        head[c] = head[v];
                    }
                    stack.push(c);
                }
            }

            Self {
                parent,
                index,
                head,
                subtree_size,
            }
        }

        pub fn index(&self, v: usize) -> usize {
            self.index[v]
        }

        pub fn edge_index(&self, u: usize, v: usize) -> usize {
            self.index[u].max(self.index[v])
        }

        pub fn subtree(&self, v: usize) -> (usize, usize) {
            let l = self.index[v];
            (l, l + self.subtree_size[v])
        }

        pub fn lca(&self, u: usize, v: usize) -> usize {
            self.path(u, v).find(|&(_, _, last)| last).unwrap().0
        }

        pub fn dist(&self, u: usize, v: usize) -> usize {
            self.path(u, v)
                .map(|(l, r, last)| usize::from(!last) + self.index[r] - self.index[l])
                .sum()
        }

        pub fn path_vertices(
            &self,
            u: usize,
            v: usize,
        ) -> impl Iterator<Item = (usize, usize)> + '_ {
            self.path(u, v)
                .map(move |(u, v, _)| (self.index[u], self.index[v] + 1))
        }

        pub fn path_edges(&self, u: usize, v: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
            self.path(u, v)
                .map(move |(u, v, last)| (self.index[u] + usize::from(last), self.index[v] + 1))
        }

        fn path(&self, u: usize, v: usize) -> PathSegments {
            PathSegments {
                hld: self,
                u,
                v,
                exhausuted: false,
            }
        }
    }

    pub struct PathSegments<'a> {
        hld: &'a HLDGraph,
        u: usize,
        v: usize,
        exhausuted: bool,
    }

    impl Iterator for PathSegments<'_> {
        type Item = (usize, usize, bool);

        fn next(&mut self) -> Option<Self::Item> {
            if self.exhausuted {
                return None;
            }

            let Self {
                hld:
                    HLDGraph {
                        parent,
                        index,
                        head,
                        ..
                    },
                u,
                v,
                ..
            } = *self;

            if head[u] == head[v] {
                self.exhausuted = true;
                if index[u] < index[v] {
                    Some((u, v, true))
                } else {
                    Some((v, u, true))
                }
            } else {
                if index[u] < index[v] {
                    self.v = parent[head[v]];
                    Some((head[v], v, false))
                } else {
                    self.u = parent[head[u]];
                    Some((head[u], u, false))
                }
            }
        }
    }
}
