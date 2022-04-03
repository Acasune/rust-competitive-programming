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
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,L:i64,
        mut S:[Chars;N],
    }
    let mut s_vec = vec![0usize; N];
    for i in 0..N {
        for &c in &S[i] {
            s_vec[i] |= 1 << (c as u8 - 'a' as u8) as usize;
        }
    }
    let mut ans = 0;
    let md = md998244353;

    for bit in 1usize..1 << N {
        let mut x = !0usize;
        for i in 0..N {
            if (bit >> i) & 1 != 0 {
                x &= s_vec[i];
            }
        }
        let k = x.count_ones();
        let a = mod_pow(k as i64, L, md);
        if bit.count_ones() & 1 != 0 {
            ans += a;
            ans %= md;
        } else {
            ans = ans + md - a;
            ans %= md;
        }
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
