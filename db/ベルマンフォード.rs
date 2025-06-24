fn main() {
    let mut dist = vec![i64::MAX; n];
    dist[0] = 0;
    for i in 0..edges.len() * 2 {
        let mut changed = false;
        for &(l, r, s) in edges.iter() {
            if dist[l] == i64::MAX {
                continue;
            }
            let d = if i < edges.len() {
                dist[l].saturating_add(s)
            } else {
                i64::MIN
            };
            if chmin(&mut dist[r], d) {
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}
