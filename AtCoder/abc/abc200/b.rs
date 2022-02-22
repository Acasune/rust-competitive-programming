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
        N:i64, K:i64
    }
    let mut N = N;
    for k in 0..K {
        if N % 200 == 0 {
            N /= 200;
        } else {
            N = N * 1_000 + 200;
        }
    }
    println!("{}", N);
}
