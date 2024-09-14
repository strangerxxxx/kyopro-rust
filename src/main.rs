// -*- coding:utf-8-unix -*-
#![allow(dead_code, unused_imports, unused_macros)]
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
const PI: f64 = 3.14159265358979323846;
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
            print!("\n");
        }
    };
}
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
fn read_mat<T: FromStr>(n: u32) -> Vec<Vec<T>> {
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
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
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
fn main() {
    input! {
     n: usize,
     k: usize,
     a: [usize; n],
    }
}
