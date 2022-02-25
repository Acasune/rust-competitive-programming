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
        A:i64, B:i64, K:i64
    }
    let mut memo = Vec::<char>::new();
    let n = calc(A, B);

    let S = dfs(&mut memo, A, B, K, n);
    println!("{}", S);
}

fn dfs(memo: &mut Vec<char>, A: i64, B: i64, K: i64, N: i64) -> String {
    if A == 0 || B == 0 {
        // println!("{} {} {}", K, A, B);
        for i in 0..A {
            memo.push('a');
        }
        for i in 0..B {
            memo.push('b')
        }
        let s: String = memo.iter().collect();
        return s;
    }
    let a = calc(A - 1, B);
    // println!("{} {} {} {}", K, a, A, B);
    // let b = calc(A - 1, B);
    if K <= a {
        memo.push('a');
        return dfs(memo, A - 1, B, K, a);
    } else {
        memo.push('b');
        return dfs(memo, A, B - 1, K - a, N - a);
    }
}
fn calc(a: i64, b: i64) -> i64 {
    if a == -1 {
        return 0;
    }
    let mut n = 1i64;
    for i in a + 1..=(a + b) {
        n *= i;
        n /= (i - a);
    }
    return n;
}
