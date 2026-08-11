fn bellman_ford(n: usize, edges: &[(usize, usize, i64)], start: usize) -> Vec<i64> {
    // dist[v] == i64::MAX: unreachable
    // dist[v] == i64::MIN: negative cycle の影響を受ける
    let mut dist = vec![i64::MAX; n];
    dist[start] = 0;

    for _ in 0..n - 1 {
        let mut changed = false;
        for &(l, r, s) in edges {
            if dist[l] == i64::MAX {
                continue;
            }
            if chmin(&mut dist[r], dist[l].saturating_add(s)) {
                changed = true;
            }
        }
        if !changed {
            return dist;
        }
    }

    // さらに緩和できる頂点は負閉路の影響を受ける
    let mut in_neg = vec![false; n];
    for &(l, r, s) in edges {
        if dist[l] == i64::MAX {
            continue;
        }
        if dist[r] > dist[l].saturating_add(s) {
            in_neg[r] = true;
        }
    }

    // 負閉路から到達可能な頂点へ伝播
    let mut g = vec![vec![]; n];
    for &(l, r, _) in edges {
        g[l].push(r);
    }
    let mut stack: Vec<usize> = (0..n).filter(|&v| in_neg[v]).collect();
    while let Some(v) = stack.pop() {
        dist[v] = i64::MIN;
        for &to in &g[v] {
            if !in_neg[to] {
                in_neg[to] = true;
                stack.push(to);
            }
        }
    }

    dist
}
