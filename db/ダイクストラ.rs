fn main() {
    let mut edges = vec![vec![]; n];
    for (s, t, d) in edge {
        edges[s].push((t, d));
    }
    let mut dists = vec![usize::MAX; n];
    dists[start] = 0;
    let mut q = BinaryHeap::<(Reverse<usize>, usize)>::from([(Reverse(0), start)]);
    while let Some((Reverse(d), i)) = q.pop() {
        if d > dists[i] {
            continue;
        }
        dists[i] = d;
        for &(j, k) in edges[i].iter() {
            let dj = d + k;
            if dists[j] <= dj {
                continue;
            }
            dists[j] = dj;
            q.push((Reverse(dj), j));
        }
    }
}
