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
        A:[i64;N],
    }
    let mut ans = 0i64;
    for i in 0..N {
        ans += 0.max(A[i] - 10);
    }
    println!("{}", ans);
}
