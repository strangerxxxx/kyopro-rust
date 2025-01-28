fn convex_hull(mut ps: Vec<(usize, usize)>, ps_is_sorted: bool) -> Vec<(usize, usize)> {
    fn cross3(a: (usize, usize), b: (usize, usize), c: (usize, usize)) -> isize {
        (b.0 as isize - a.0 as isize) * (c.1 as isize - a.1 as isize)
            - (b.1 as isize - a.1 as isize) * (c.0 as isize - a.0 as isize)
    }

    if !ps_is_sorted {
        ps.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut res = Vec::new();

    for &p in &ps {
        while res.len() > 1 && cross3(res[res.len() - 2], res[res.len() - 1], p) > 0 {
            res.pop();
        }
        res.push(p);
    }

    let t = res.len();

    for i in (0..ps.len() - 1).rev() {
        let p = ps[i];
        while res.len() > t && cross3(res[res.len() - 2], res[res.len() - 1], p) > 0 {
            res.pop();
        }
        res.push(p);
    }

    res
}
