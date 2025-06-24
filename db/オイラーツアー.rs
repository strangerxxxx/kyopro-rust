fn main() {
    let mut edges = vec![vec![]; n];
    for &(u, v) in &edge {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut route = vec![];
    let mut q = vec![(0, !0), (!0, 0)];
    while let Some((pre, i)) = q.pop() {
        if i < n {
            left[i] = route.len();
            route.push(i);
            for j in edges[i].iter() {
                if *j != pre {
                    q.push((i, !j));
                    q.push((i, *j));
                }
            }
        } else {
            right[!i] = route.len();
            route.push(!i);
        }
    }
}
fn main() {
    let mut edges = vec![vec![]; n];
    for &(u, v) in &edge {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut route = vec![];
    fn dfs(
        i: usize,
        pre: usize,
        route: &mut Vec<usize>,
        left: &mut Vec<usize>,
        right: &mut Vec<usize>,
        edges: &Vec<Vec<usize>>,
    ) {
        left[i] = route.len();
        route.push(i);
        for j in edges[i].iter() {
            if *j != pre {
                dfs(*j, i, route, left, right, &edges);
            }
        }
        right[i] = route.len();
        route.push(i);
    }
    dfs(0, !0, &mut route, &mut left, &mut right, &edges);
}
