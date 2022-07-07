#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:i64,K:i64,
    }
    let mut cnt = vec![0; 1_000_000];
    for g in (1..=K).rev() {
        let val = mod_pow(K / g, N, md1000000007);
        cnt[g as usize] = val;
        let mut gg = 2 * g;
        while gg <= K {
            cnt[g as usize] = (md1000000007 + cnt[g as usize] - cnt[gg as usize]) % md1000000007;
            gg += g;
        }
    }
    let mut ans = 0;
    for i in 1..=K {
        ans += i * cnt[i as usize];
        ans %= md1000000007;
    }
    println!("{}", ans);
}

fn mod_pow(base: i64, power: i64, md: i64) -> i64 {
    // base^power % md
    let mut ret = 1;
    let mut base = base;
    let mut power = power;
    while power > 0 {
        if power % 2 == 1 {
            ret = (ret % md) * (base % md);
            ret %= md;
        }
        base = (base % md) * (base % md);
        base %= md;
        power >>= 1;
    }
    return ret;
}
