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
        N:usize,Q:usize,
        A:[usize;N],
        K:[usize;Q],
    }
    let mut cumsum = vec![0; N + 1];
    cumsum[0] = A[0] - 1;
    for i in 1..N {
        cumsum[i] = cumsum[i - 1] + A[i] - A[i - 1] - 1;
    }
    cumsum[N] = 1_000_000_000_000_000_100;
    // println!("{:?}", cumsum);
    for k in K {
        let idx = cumsum
            .binary_search_by_key(&(2 * k), |&a| 2 * a + 1)
            .err()
            .unwrap();
        // println!("{}", idx);
        if idx == 0 {
            println!("{}", k);
        } else {
            println!("{}", A[idx - 1] + k - cumsum[idx - 1]);
        }
    }
}
