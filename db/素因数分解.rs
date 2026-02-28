pub struct PrimeFactorization {
    spf: Vec<usize>,
}

impl PrimeFactorization {
    pub fn new(n: usize) -> Self {
        let mut spf: Vec<usize> = (0..=n).collect();

        for i in 2..=n.isqrt() {
            if spf[i] == i {
                for j in (i * i..=n).step_by(i) {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                }
            }
        }

        Self { spf }
    }

    pub fn factor(&self, mut x: usize) -> HashMap<usize, usize> {
        let mut res = HashMap::new();
        while x > 1 {
            let p = self.spf[x];
            *res.entry(p).or_insert(0) += 1;
            x /= p;
        }
        res
    }
}
