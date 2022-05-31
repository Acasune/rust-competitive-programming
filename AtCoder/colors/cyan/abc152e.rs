#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:usize,
         A:[usize;N]
    }
    let mut mp = HashMap::new();
    let mut all_use = HashMap::new();
    for (i, &a) in A.iter().enumerate() {
        prime_factorize(a, &mut mp, &mut all_use);
    }

    // println!("{:?}", mp);

    let md = md1000000007;
    let mut ans = 0;
    let mut lcm = 1;

    for (&key, &val) in &mp {
        lcm *= mod_pow(key as i64, val as i64, md);
        lcm %= md;
    }
    for &a in &A {
        ans += lcm * mod_pow(a as i64, md as i64 - 2, md);
        ans %= md;
    }

    println!("{}", ans);
}

fn prime_factorize(n: usize, mp: &mut HashMap<usize, usize>, all_use: &mut HashMap<usize, usize>) {
    let mut i = 2;
    let mut n = n;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
            continue;
        }
        let mut cnt = 0;
        while n % i == 0 {
            cnt += 1;
            n /= i;
        }
        if cnt != 0 {
            if !mp.contains_key(&i) {
                mp.insert(i, cnt);
            } else if let Some(&g_cnt) = mp.get(&i) {
                if g_cnt < cnt {
                    mp.insert(i, cnt);
                }
            }
        }
        i += 1;
    }
    if n != 1 && !mp.contains_key(&n) {
        mp.insert(n, 1);
    }
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
