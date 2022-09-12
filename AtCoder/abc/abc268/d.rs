#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        N:usize,M:usize,
        S:[String;N],
        T:[String;M],
    }
    let len = S.iter().map(|s| s.len()).sum::<usize>();

    let rem = 16 - len;
    let mut memo = vec![];
    let mut vec = vec![0; N - 1];
    if rem < N - 1 {
        println!("{}", -1);
        return;
    }
    dfs(&mut memo, &mut vec, 0, rem + 1 - N, N);

    let mut set = HashSet::<String>::new();
    for s in S.into_iter().permutations(N) {
        for mi in 0..memo.len() {
            let m = memo[mi].clone();
            let mut st = "".to_string();
            for i in 0..N - 1 {
                st.push_str(&s[i]);
                st.push_str(&m[i]);
            }
            st.push_str(&s[N - 1]);
            if 16 < st.len() || st.len() < 3 {
                continue;
            }
            set.insert(st);
        }
    }
    for t in T {
        set.remove(&t);
    }
    if set.len() == 0 {
        println!("{}", -1);
    } else {
        for s in set.into_iter() {
            println!("{}", s);
            return;
        }
    }
}

fn dfs(memo: &mut Vec<Vec<String>>, nums: &mut Vec<usize>, i: usize, remain: usize, N: usize) {
    if i == N - 1 {
        let mut unders = vec![];
        for num in nums {
            unders.push(vec!['_'; num.clone() + 1].into_iter().collect::<String>());
        }
        memo.push(unders);
        return;
    }
    for cnt in 0..=remain {
        nums[i] = cnt;
        dfs(memo, nums, i + 1, remain - cnt, N);
    }
}
