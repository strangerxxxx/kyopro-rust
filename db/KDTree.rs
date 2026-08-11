type Point = (i64, i64);

#[derive(Debug)]
struct KDTree {
    l: Option<Box<KDTree>>,
    r: Option<Box<KDTree>>,
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    size: usize,
    weight: usize,
    data: Vec<Point>,
}

impl KDTree {
    fn new(points: &mut [Point], divx: bool) -> Self {
        let mut xmin = i64::MAX;
        let mut xmax = i64::MIN;
        let mut ymin = i64::MAX;
        let mut ymax = i64::MIN;
        let mut weight = 0;

        for &(x, y) in points.iter() {
            chmin(&mut xmin, x);
            chmax(&mut xmax, x);
            chmin(&mut ymin, y);
            chmax(&mut ymax, y);
            weight += 1;
        }

        let size = points.len();
        if size <= 1 {
            return KDTree {
                l: None,
                r: None,
                xmin,
                xmax,
                ymin,
                ymax,
                size,
                weight,
                data: points.to_vec(),
            };
        }

        let mid = size / 2;
        if divx {
            points.sort_by_key(|&(x, _)| x);
        } else {
            points.sort_by_key(|&(_, y)| y);
        }

        let (left, right) = points.split_at_mut(mid);

        KDTree {
            l: Some(Box::new(KDTree::new(left, !divx))),
            r: Some(Box::new(KDTree::new(right, !divx))),
            xmin,
            xmax,
            ymin,
            ymax,
            size,
            weight,
            data: vec![],
        }
    }

    fn in_range(p: Point, x1: i64, x2: i64, y1: i64, y2: i64) -> bool {
        let (x, y) = p;
        x1 <= x && x <= x2 && y1 <= y && y <= y2
    }

    fn count(&self, x1: i64, x2: i64, y1: i64, y2: i64) -> usize {
        if x2 < self.xmin || self.xmax < x1 || y2 < self.ymin || self.ymax < y1 {
            return 0;
        }
        if x1 <= self.xmin && self.xmax <= x2 && y1 <= self.ymin && self.ymax <= y2 {
            return self.size;
        }
        if self.l.is_none() && self.r.is_none() {
            return self
                .data
                .iter()
                .filter(|&&p| Self::in_range(p, x1, x2, y1, y2))
                .count();
        }
        self.l.as_ref().map_or(0, |l| l.count(x1, x2, y1, y2))
            + self.r.as_ref().map_or(0, |r| r.count(x1, x2, y1, y2))
    }

    fn sum(&self, x1: i64, x2: i64, y1: i64, y2: i64) -> usize {
        // weight は点数なので、現状は count と同値
        if x2 < self.xmin || self.xmax < x1 || y2 < self.ymin || self.ymax < y1 {
            return 0;
        }
        if x1 <= self.xmin && self.xmax <= x2 && y1 <= self.ymin && self.ymax <= y2 {
            return self.weight;
        }
        if self.l.is_none() && self.r.is_none() {
            return self
                .data
                .iter()
                .filter(|&&p| Self::in_range(p, x1, x2, y1, y2))
                .count();
        }
        self.l.as_ref().map_or(0, |l| l.sum(x1, x2, y1, y2))
            + self.r.as_ref().map_or(0, |r| r.sum(x1, x2, y1, y2))
    }

    fn points(&self, x1: i64, x2: i64, y1: i64, y2: i64) -> Vec<Point> {
        if x2 < self.xmin || self.xmax < x1 || y2 < self.ymin || self.ymax < y1 {
            return vec![];
        }
        if x1 <= self.xmin && self.xmax <= x2 && y1 <= self.ymin && self.ymax <= y2 {
            return self.collect_all_points();
        }
        if self.l.is_none() && self.r.is_none() {
            return self
                .data
                .iter()
                .copied()
                .filter(|&p| Self::in_range(p, x1, x2, y1, y2))
                .collect();
        }
        let mut res = vec![];
        if let Some(l) = &self.l {
            res.extend(l.points(x1, x2, y1, y2));
        }
        if let Some(r) = &self.r {
            res.extend(r.points(x1, x2, y1, y2));
        }
        res
    }

    fn collect_all_points(&self) -> Vec<Point> {
        if self.l.is_none() && self.r.is_none() {
            return self.data.clone();
        }
        let mut res = vec![];
        if let Some(l) = &self.l {
            res.extend(l.collect_all_points());
        }
        if let Some(r) = &self.r {
            res.extend(r.collect_all_points());
        }
        res
    }
}
