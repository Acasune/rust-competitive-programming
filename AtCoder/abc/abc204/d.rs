#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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
        mut A:[usize;N],
    }
    let mut sum = A.iter().sum::<usize>();

    let mut dp = vec![false; sum + 1];
    dp[0] = true;
    for j in 0..N {
        for i in (0..=sum).rev() {
            let idx = A[j];
            if sum < i + idx {
                continue;
            }
            dp[idx + i] |= dp[i];
        }
    }
    for i in (sum + 1) / 2..=sum {
        if dp[i] {
            println!("{}", i);
            return;
        }
    }
}
