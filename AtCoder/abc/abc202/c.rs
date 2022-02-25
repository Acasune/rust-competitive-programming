#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

fn main() {
    input! {
        N:usize,
        A:[i64;N], B:[i64;N], C:[usize;N],
    }
    let mut memo = vec![0i64; N + 1];
    for &a in &A {
        memo[a as usize] += 1
    }
    let mut ans = 0;
    for &c in &C {
        ans += memo[B[c - 1] as usize];
        // println!("{} {}", c - 1, C[c - 1]);
    }
    println!("{}", ans);
}
