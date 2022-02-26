#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::{HashMap, VecDeque};
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
        N:usize, M:usize,
        AB:[[usize;2];M],
    }
    let mut roads = vec![Vec::<usize>::new(); N];
    for i in 0..M {
        roads[AB[i][0] - 1].push(AB[i][1] - 1);
    }
    let mut ans = 0i64;
    for i in 0..N {
        let mut memo = vec![false; N];
        memo[i] = true;
        let mut que = VecDeque::<usize>::new();
        que.push_back(i);
        while !que.is_empty() {
            let s = que.pop_back().unwrap();
            memo[s] = true;
            for &e in &roads[s] {
                if !memo[e] {
                    que.push_back(e);
                }
            }
        }
        ans += memo.iter().filter(|&x| *x).count() as i64;
    }
    println!("{}", ans);
}
