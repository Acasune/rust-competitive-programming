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
        N:usize, K:usize,
        A:[usize;N],
    }
    let mut memo = vec![0; N];
    let mut k = 1usize;
    let mut now = 0usize;
    let mut ans = 0usize;
    let mut visited = vec![0; N];
    let mut is_passed = false;
    while k <= K {
        if visited[now] > 0 && !is_passed {
            let cycle_size = k - visited[now];
            let n_candy = ans - memo[now];
            let n_cycle = (K - k) / cycle_size;
            ans += n_candy * n_cycle;
            k = k + n_cycle * cycle_size;
            is_passed = true;
        }

        visited[now] = k;
        memo[now] = ans;
        ans += A[now];
        now = ans % N;
        k += 1;
    }
    println!("{}", ans);
}
