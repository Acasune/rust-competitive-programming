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
        A: [i64;N],
    }
    let mut memo = vec![0; 200];
    for &a in &A {
        memo[(a % 200) as usize] += 1;
    }

    let mut ans: i64 = 0;
    for i in 0..200 {
        if memo[i] > 0 {
            ans += (memo[i] * (memo[i] - 1)) / 2;
        }
    }

    println!("{}", ans);
}
