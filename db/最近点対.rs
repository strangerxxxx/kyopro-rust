// -*- coding:utf-8-unix -*-
#![allow(dead_code, unused_imports, unused_macros)]
fn main() {
    input! {
        n: usize,
        mut points: [(f64, f64); n],
    }
    fn merge(p: Vec<(f64, f64)>, q: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        let mut p = p.clone();
        p.reverse();
        let mut q = q.clone();
        q.reverse();
        let mut res = vec![];
        while !p.is_empty() && !q.is_empty() {
            if p.last().unwrap().1 <= q.last().unwrap().1 {
                res.push(p.pop().unwrap());
            } else {
                res.push(q.pop().unwrap());
            }
        }
        while !p.is_empty() {
            res.push(p.pop().unwrap());
        }
        while !q.is_empty() {
            res.push(q.pop().unwrap());
        }
        res
    }
    fn euclidean_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
        ((point2.0 - point1.0).powi(2) + (point2.1 - point1.1).powi(2)).sqrt()
    }
    fn cp_rec(p: &Vec<(f64, f64)>, l: usize, r: usize) -> (f64, Vec<(f64, f64)>) {
        if r - l <= 1 {
            let res: Vec<(f64, f64)> = p[l..r].iter().cloned().collect();
            return (f64::MAX, res);
        }
        let mid = (r + l) / 2;
        let mid_x = p[mid].0;
        let (d1, q1) = cp_rec(p, l, mid);
        let (d2, q2) = cp_rec(p, mid, r);
        let q = merge(q1, q2);
        let mut dist = if d1 <= d2 { d1 } else { d2 };
        for (idx, &(x, y)) in q.iter().enumerate() {
            if (x - mid_x).abs() >= dist {
                continue;
            }
            for &(x2, y2) in q[..idx].iter().rev() {
                if (y2 - y).abs() >= dist {
                    break;
                }
                let d2 = euclidean_distance((x, y), (x2, y2));
                if d2 < dist {
                    dist = d2;
                }
            }
        }
        (dist, q)
    }
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let (ans, _) = cp_rec(&points, 0, n);
    p!(ans);
}
use ::num;
// use ac_library::*;
// use proconio::input;
use cmp::Ordering::*;
// use itertools::Itertools;
// use itertools_num::ItertoolsNum;
// use maplit;
// use ordered_float::OrderedFloat;
use std::cmp::*;
use std::collections::*;
use std::fmt::*;
use std::hash::*;
use std::io::{stdin, stdout, Write};
use std::iter::FromIterator;
use std::str::FromStr;
use std::*;
// use superslice::*;
// use rand::Rng;
// use rand::rngs::SmallRng;
// use rand::seq::SliceRandom;
const MOD1000000007: i64 = 1000000007;
const MOD998244353: i64 = 998244353;
const MOD: i64 = 998244353;
const UMOD: usize = MOD as usize;
const PI: f64 = f64::consts::PI;
const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
#[macro_export]
macro_rules! p {
    ($($arg:expr),*) => {
        {
            print!("{}\n", vec![$(format!("{}", $arg)),*].join(" "));
        }
    };
}
#[macro_export]
macro_rules! vp {
    ($x:expr) => {
        print!(
            "{}\n",
            $x.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    };
}
#[macro_export]
macro_rules! dprint {
    ($($arg:expr),*) => {
        #[allow(unused_assignments)]
        {
            let mut first = true;
            $(
                if !first {
                    eprint!(" ");
                }
                eprint!("{:?}", $arg);
                first = false;
            )*
            eprint!("\n");
        }
    };
}
#[macro_export]
macro_rules! yesno {
    ($val:expr) => {
        if $val {
            print!("Yes\n");
        } else {
            print!("No\n");
        }
    };
}
fn read<T: FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
fn read_vec<T: FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
fn readii() -> (i64, i64) {
    let vec: Vec<i64> = read_vec();
    (vec[0], vec[1])
}
fn readiii() -> (i64, i64, i64) {
    let vec: Vec<i64> = read_vec();
    (vec[0], vec[1], vec[2])
}
fn readuu() -> (usize, usize) {
    let vec: Vec<usize> = read_vec();
    (vec[0], vec[1])
}
fn readff() -> (f64, f64) {
    let vec: Vec<f64> = read_vec();
    (vec[0], vec[1])
}
fn readcc() -> (char, char) {
    let vec: Vec<char> = read_vec();
    (vec[0], vec[1])
}
fn readuuu() -> (usize, usize, usize) {
    let vec: Vec<usize> = read_vec();
    (vec[0], vec[1], vec[2])
}
fn readiiii() -> (i64, i64, i64, i64) {
    let vec: Vec<i64> = read_vec();
    (vec[0], vec[1], vec[2], vec[3])
}
fn readuuuu() -> (usize, usize, usize, usize) {
    let vec: Vec<usize> = read_vec();
    (vec[0], vec[1], vec[2], vec[3])
}
#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
#[macro_export]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
#[macro_export]
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
