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
        N:i64,  K:i64,
    }
    let mut ans = 0;
    for a in 1..=N {
        for b in 1..=K {
            ans += 100 * a + b;
        }
    }

    println!("{}", ans);
}
