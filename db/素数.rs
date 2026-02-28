use std::collections::HashMap;
pub fn faster_eratosthenes(n: usize) -> Vec<usize> {
    if n < 30 {
        return vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
            .into_iter()
            .filter(|&x| x <= n)
            .collect();
    }

    let remains = vec![1, 7, 11, 13, 17, 19, 23, 29];

    let div30 = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 4, 5, 6, 0, 2, 4, 4, 6, 6, 8, 10, 0, 3, 4, 5, 7, 8,
        9, 12, 0, 3, 6, 7, 9, 10, 13, 16, 0, 4, 6, 8, 10, 12, 14, 18, 0, 5, 8, 9, 13, 14, 17, 22,
        0, 6, 10, 12, 16, 18, 22, 28,
    ];

    let mod30 = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 1, 5, 4, 0, 7, 3, 2, 6, 2, 4, 0, 6, 1, 7, 3, 5, 3, 0, 6, 5, 2, 1,
        7, 4, 4, 7, 1, 2, 5, 6, 0, 3, 5, 3, 7, 1, 6, 0, 4, 2, 6, 2, 3, 7, 0, 4, 5, 1, 7, 6, 5, 4,
        3, 2, 1, 0,
    ];

    let msk: u8 = 255;
    let shift: Vec<u8> = (0..8).map(|i| 1 << i).collect();
    let msk8: Vec<u8> = shift.iter().map(|&s| msk - s).collect();

    let inv_shift: HashMap<u8, usize> = shift.iter().enumerate().map(|(i, &s)| (s, i)).collect();

    let mut res = vec![2, 3, 5];
    let max_k = n / 30;

    let sqrtn = (n as f64).sqrt().ceil() as usize;
    let max_m = remains.iter().take_while(|&&r| r <= n % 30).count() - 1;
    let max_sqrt_k = sqrtn / 30;
    let max_sqrt_m = remains.iter().take_while(|&&r| r <= sqrtn % 30).count() - 1;

    let mut table = vec![msk; max_k + 1];
    table[max_k] = (1 << (max_m + 1)) - 1;
    table[0] -= 1;

    for k in 0..=max_sqrt_k {
        for m in 0..8 {
            if k == max_sqrt_k && m > max_sqrt_m {
                break;
            }
            if table[k] & shift[m] != 0 {
                let mut m_before = m;
                let mut i = k * (30 * k + 2 * remains[m]) + div30[(m << 3) + m];
                let mut j = mod30[(m << 3) + m];
                while i < max_k || (i == max_k && j <= max_m) {
                    table[i] &= msk8[j];
                    if m_before == 7 {
                        i += 2 * k + remains[m] + div30[m << 3] - div30[(m << 3) + 7];
                        j = mod30[m << 3];
                        m_before = 0;
                    } else {
                        i += k * (remains[m_before + 1] - remains[m_before])
                            + div30[(m << 3) + m_before + 1]
                            - div30[(m << 3) + m_before];
                        j = mod30[(m << 3) + m_before + 1];
                        m_before += 1;
                    }
                }
            }
        }
    }

    let mut i30 = 0;
    for &value in &table {
        let mut bits = value;
        while bits != 0 {
            let j = inv_shift[&(bits & (!bits + 1))];
            res.push(i30 + remains[j]);
            bits &= bits - 1;
        }
        i30 += 30;
    }

    res
}
