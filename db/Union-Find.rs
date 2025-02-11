#[derive(Clone, Debug)]
pub struct UnionFind {
    n: usize,
    parent: Vec<i32>,
    groups: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            n,
            parent: vec![-1; n],
            groups: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let mut p = x;
        while self.parent[p] >= 0 {
            p = self.parent[p] as usize;
        }
        let root = p;
        let mut x_copy = x;
        while self.parent[x_copy] >= 0 {
            let next = self.parent[x_copy] as usize;
            self.parent[x_copy] = root as i32;
            x_copy = next;
        }
        root
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.find(x);
        let mut y_root = self.find(y);
        if x_root == y_root {
            return false;
        }
        if self.parent[x_root] > self.parent[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }
        self.parent[x_root] += self.parent[y_root];
        self.parent[y_root] = x_root as i32;
        self.groups -= 1;
        true
    }

    pub fn union_left(&mut self, parent: usize, child: usize) -> bool {
        let parent_root = self.find(parent);
        let child_root = self.find(child);
        if parent_root == child_root {
            return false;
        }
        self.parent[parent_root] += self.parent[child_root];
        self.parent[child_root] = parent_root as i32;
        self.groups -= 1;
        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.n).filter(|&i| self.find(i) == root).collect()
    }

    pub fn roots(&self) -> Vec<usize> {
        self.parent
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| if p < 0 { Some(i) } else { None })
            .collect()
    }

    pub fn group_count(&self) -> usize {
        self.groups
    }

    pub fn sizes(&self) -> HashMap<usize, usize> {
        let mut sizes = HashMap::new();
        for (i, &p) in self.parent.iter().enumerate() {
            if p < 0 {
                sizes.insert(i, -p as usize);
            }
        }
        sizes
    }

    pub fn add_member(&mut self) -> usize {
        self.n += 1;
        self.groups += 1;
        self.parent.push(-1);
        self.n - 1
    }

    pub fn all_group_members(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..self.n {
            let root = self.find(i);
            groups.entry(root).or_insert(Vec::new()).push(i);
        }
        groups
    }
}

impl std::fmt::Display for UnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut union_find_copy = self.clone();
        let groups = union_find_copy.all_group_members();
        for (k, v) in &groups {
            writeln!(f, "{}: {:?}", k, v)?;
        }
        Ok(())
    }
}
