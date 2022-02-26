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
        N:usize,K:usize,
        A:[[i64;N];N],
    }

    let mut ok = 0;
    let mut ng = 1_000_000_000_000i64;
    let lim = (K * K) / 2 + 1;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;
        let mut cs = vec![vec![0; N + 1]; N + 1];
        for i in 0..N {
            for j in 0..N {
                cs[j + 1][i + 1] = cs[j + 1][i] + cs[j][i + 1] - cs[j][i];
                if A[j][i] >= mid {
                    cs[j + 1][i + 1] += 1;
                }
            }
        }

        let mut is_ok = true;

        for j in 0..=N - K {
            for i in 0..=N - K {
                if cs[j + K][i + K] - cs[j][i + K] - cs[j + K][i] + cs[j][i] < lim {
                    is_ok = false;
                }
            }
        }
        if is_ok {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
