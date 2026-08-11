use std::collections::HashMap;

fn pow_mod(mut x: i64, mut n: i64, mod_: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n & 1 != 0 {
            res = res * x % mod_;
        }
        x = x * x % mod_;
        n >>= 1;
    }
    res
}

fn cmb(n: i64, r: i64, mod_: i64) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    if r > n - r {
        return cmb(n, n - r, mod_);
    }
    let mut c = 1;
    let mut d = 1;
    for i in 0..r {
        c = c * (n - i) % mod_;
        d = d * (r - i) % mod_;
    }
    c * pow_mod(d, mod_ - 2, mod_) % mod_
}

fn perm(mut n: i64, mut r: i64, mod_: i64) -> i64 {
    if r == -1 {
        r = n;
    }
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let mut res = 1;
    for _ in 0..r {
        res = res * n % mod_;
        n -= 1;
    }
    res
}

fn hom(n: i64, r: i64, mod_: i64) -> i64 {
    if n == 0 {
        return if r == 0 { 1 } else { 0 };
    }
    if r < 0 {
        return 0;
    }
    cmb(n + r - 1, r, mod_)
}

struct Combination {
    n_max: i64,
    mod_: i64,
    modinv: Vec<i64>,
    fac: Vec<i64>,
    facinv: Vec<i64>,
}

impl Combination {
    fn new(maxn: i64, mod_: i64) -> Combination {
        let modinv = Combination::make_modinv_list(maxn, mod_);
        let (fac, facinv) = Combination::make_factorial_list(maxn, mod_, &modinv);
        Combination {
            n_max: maxn,
            mod_: mod_,
            modinv,
            fac,
            facinv,
        }
    }

    fn make_modinv_list(n: i64, mod_: i64) -> Vec<i64> {
        let mut res = vec![0; (n + 1) as usize];
        res[1] = 1;
        for i in 2..=n {
            res[i as usize] = mod_ - res[(mod_ as usize % i as usize)] * (mod_ / i) % mod_;
        }
        res
    }

    fn make_factorial_list(n: i64, mod_: i64, modinv: &Vec<i64>) -> (Vec<i64>, Vec<i64>) {
        let mut fac = vec![1; (n + 1) as usize];
        let mut facinv = vec![1; (n + 1) as usize];
        for i in 1..=n {
            fac[i as usize] = fac[i as usize - 1] * i % mod_;
            facinv[i as usize] = facinv[i as usize - 1] * modinv[i as usize] % mod_;
        }
        (fac, facinv)
    }

    fn calc(&self, n: i64, r: i64) -> i64 {
        if n < 0 || r < 0 || n < r {
            return 0;
        }
        assert!(n <= self.n_max);
        self.fac[n as usize] * self.facinv[r as usize] % self.mod_ * self.facinv[(n - r) as usize]
            % self.mod_
    }
}

struct MemorizeCombination {
    cmb: HashMap<(i64, i64, i64), i64>,
}

impl MemorizeCombination {
    fn new() -> Self {
        Self {
            cmb: HashMap::new(),
        }
    }

    fn call(&mut self, n: i64, r: i64, m: i64) -> i64 {
        if r < 0 || n < r {
            return 0;
        }
        let r = if r * 2 > n { n - r } else { r };
        if r == 0 {
            return 1;
        }
        if let Some(&res) = self.cmb.get(&(n, r, m)) {
            return res;
        }
        let res = (self.call(n - 1, r - 1, m) + self.call(n - 1, r, m)) % m;
        self.cmb.insert((n, r, m), res);
        res
    }

    fn init_calc(&mut self, n: i64, r: i64, m: i64) {
        for i in 0..=n {
            for j in 0..=std::cmp::min(i, r) {
                self.call(i, j, m);
            }
        }
    }
}
