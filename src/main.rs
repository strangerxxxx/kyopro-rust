// -*- coding:utf-8-unix -*-
#![allow(dead_code, unused_imports, unused_macros)]
fn main() {
    input! {
     n: usize,
     m: usize,
     a: [usize; n],
    }
}
// use ::num;
// use ac_library::*;
use cmp::Ordering::*;
// use fixedbitset::FixedBitSet;
// use itertools::Itertools;
// use itertools_num::ItertoolsNum;
// use maplit;
// use omniswap::swap;
// use ordered_float::OrderedFloat;
// use proconio::input;
// use rand::rngs::SmallRng;
// use rand::seq::SliceRandom;
// use rand::Rng;
use std::cmp::*;
use std::collections::*;
use std::fmt::*;
use std::hash::*;
use std::io::{stdin, stdout, Write};
use std::iter::FromIterator;
use std::str::FromStr;
use std::*;
// use superslice::*;
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
        {
            eprint!("{}\n", vec![$(format!("{:?}", $arg)),*].join(" "));
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
fn chmin<T: PartialOrd + Copy>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}
fn chmax<T: PartialOrd + Copy>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}
pub trait BinarySearch<T> {
    fn bisect_left(&self, key: T) -> usize;
    fn bisect_right(&self, key: T) -> usize;
}
impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    fn bisect_left(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    fn bisect_right(&self, key: T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if key < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}
fn mod_pow<T>(x: T, a: T, md: T) -> T
where
    T: Copy
        + From<u8>
        + std::ops::Mul<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::Shr<u64, Output = T>
        + std::ops::Add<Output = T>
        + std::ops::BitAnd<Output = T>
        + PartialOrd
        + Default
        + std::ops::ShrAssign<i32>,
{
    let mut res = T::from(1);
    let mut base = x;
    let mut ai = a;

    while ai > T::from(0) {
        if ai & T::from(1) == T::from(1) {
            res = res * base % md;
        }
        ai >>= 1;
        base = (base * base) % md;
    }
    res
}
