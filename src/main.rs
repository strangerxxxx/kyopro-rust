// -*- coding:utf-8-unix -*-
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::cmp::*;
use std::collections::*;

// ---------------------------------------------------------------------------
// I/O: proconio があればそれを使い、なければ同 API のフォールバック input! を使う
// - AtCoder ジャッジ: --cfg atcoder → proconio（ジャッジ側 Cargo.toml に依存あり）
// - ローカル既定: feature use-proconio → proconio
// - proconio なし: cargo build --no-default-features  /  単体 rustc
// ---------------------------------------------------------------------------
#[cfg(any(atcoder, feature = "use-proconio"))]
use proconio::input;

// proconio 非利用時のフォールバック（tanakh input! 互換）
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[cfg(not(any(atcoder, feature = "use-proconio")))]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || iter.next().expect("input token missing");
        input_inner! { next, $($r)* }
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String {
            bytes
                .by_ref()
                .map(|r| r.expect("stdin read error") as char)
                .skip_while(|c| c.is_whitespace())
                .take_while(|c| !c.is_whitespace())
                .collect()
        };
        input_inner! { next, $($r)* }
    };
}

#[cfg(not(any(atcoder, feature = "use-proconio")))]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner! { $next $($r)* }
    };
}

#[cfg(not(any(atcoder, feature = "use-proconio")))]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const INF32: i32 = 1_010_101_010;
const UINF32: u32 = 2_020_202_020;
const IINF32: i32 = -INF32;
const INF64: i64 = 4_040_404_040_404_040_404;
const UINF64: u64 = 8_080_808_080_808_080_808;
const IINF64: i64 = -INF64;
const INF128: i128 = i128::MAX / 4;
const IINF128: i128 = -INF128;
const MOD1000000007: i64 = 1_000_000_007;
const MOD998244353: i64 = 998_244_353;
const MOD: i64 = 998_244_353;
const UMOD: usize = MOD as usize;
const PI: f64 = std::f64::consts::PI;

macro_rules! p {
    ($($arg:expr),*) => {
        #[allow(unused_assignments)]
        {
            let mut first = true;
            $(
                if !first {
                    print!(" ");
                }
                print!("{}", $arg);
                first = false;
            )*
            println!();
        }
    };
}

macro_rules! vp {
    ($x:expr) => {{
        let mut first = true;
        for x in &$x {
            if !first {
                print!(" ");
            }
            print!("{}", x);
            first = false;
        }
        println!();
    }};
}

#[cfg(not(atcoder))]
macro_rules! dprint {
    ($x:expr) => {
        eprintln!("{:?}", $x);
    };
}
#[cfg(atcoder)]
macro_rules! dprint {
    ($x:expr) => {};
}

macro_rules! yesno {
    ($val:expr) => {
        if $val {
            println!("Yes");
        } else {
            println!("No");
        }
    };
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let _ = (n, k, a);
}
