fn make_divisors(n: usize) -> Vec<usize> {
    let mut lower_divisors = vec![];
    let mut upper_divisors = vec![];
    for i in 1..=u64_floor_sqrt(n) {
        if n % i == 0 {
            lower_divisors.push(i);
            if i != n / i {
                upper_divisors.push(n / i);
            }
        }
    }
    upper_divisors.reverse();
    lower_divisors.extend(upper_divisors);
    lower_divisors
}
fn u64_floor_sqrt(n: usize) -> usize {
    let tmp = (n as f64).sqrt() as usize;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
    }
}
