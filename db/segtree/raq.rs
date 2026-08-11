pub struct BIT {
    n: usize,
    bit: [Vec<i64>; 2],
}

impl BIT {
    pub fn new(n: usize) -> Self {
        let size = n + 2;
        Self {
            n,
            bit: [vec![0; size], vec![0; size]],
        }
    }

    fn add_sub(&mut self, p: usize, i: usize, x: i64) {
        let mut idx = i + 1;
        while idx < self.bit[p].len() {
            self.bit[p][idx] += x;
            idx += idx & (!idx + 1);
        }
    }

    /// 区間 [l, r) に x を加算（0-indexed）
    pub fn add(&mut self, l: usize, r: usize, x: i64) {
        let lval = if l > 0 { -x * (l as i64 - 1) } else { x };
        let rval = x * (r as i64 - 1);
        self.add_sub(0, l, lval);
        self.add_sub(0, r, rval);
        self.add_sub(1, l, x);
        self.add_sub(1, r, -x);
    }

    fn sum_sub(&self, p: usize, i: usize) -> i64 {
        let mut s = 0;
        let mut idx = i + 1;
        while idx > 0 {
            s += self.bit[p][idx];
            idx -= idx & idx.wrapping_neg();
        }
        s
    }

    /// 区間 [0, i] の累積和（0-indexed）→ i を 1 加えて使う
    pub fn sum(&self, i: usize) -> i64 {
        self.sum_sub(0, i) + self.sum_sub(1, i) * (i as i64)
    }

    /// 位置 i の値を取得（0-indexed）
    pub fn point_get(&self, i: usize) -> i64 {
        if i == 0 {
            self.sum(0)
        } else {
            self.sum(i) - self.sum(i - 1)
        }
    }
}
