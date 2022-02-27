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
        N:usize,K:i64,
        mut A:[i64;N],
    }
    A.sort();
    let mut ok = 0;
    let mut ng = 1_000_000_010i64;
    let mut ans = ng;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;
        let cnt = A.iter().fold(0, |sum, x| sum + (x + mid - 1) / mid - 1);
        if cnt <= K {
            ng = mid;
        } else {
            ok = mid;
        }
    }
    println!("{}", ok + 1);
}
