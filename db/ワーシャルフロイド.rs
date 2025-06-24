fn main() {
    let mut d = vec![vec![i64::MAX; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for (s, t, di) in edges {
        d[s][t] = di;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] != i64::MAX && d[k][j] != i64::MAX {
                    let x = d[i][k] + d[k][j];
                    chmin(&mut d[i][j], x);
                }
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] != i64::MAX && d[k][j] != i64::MAX {
                    let x = d[i][k] + d[k][j];
                    if d[i][j] > x {
                        p!("NEGATIVE CYCLE");
                        return;
                    }
                }
            }
        }
    }
}
