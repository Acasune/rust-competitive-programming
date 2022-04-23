#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

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
        N:usize,K:usize,
        mut S:[Bytes;N],
    }
    let mut memo = vec![vec![0usize; 26]; N];
    for i in 0..N {
        for c in 'a' as u8..='z' as u8 {
            for &c2 in &S[i] {
                if c == c2 {
                    memo[i][(c - 'a' as u8) as usize] = 1;
                }
            }
        }
    }
    let mut ans = 0;
    for bit in 0..(1 << N) {
        let mut flg = true;
        let mut val = 0;
        for c in 'a' as u8..='z' as u8 {
            let mut cnt = 0;
            for i in 0..N {
                if (bit >> i) & 1 != 0 {
                    cnt += memo[i][(c - 'a' as u8) as usize]
                }
            }
            if cnt == K {
                val += 1;
            }
        }
        ans = ans.max(val);
    }
    println!("{}", ans);
}
