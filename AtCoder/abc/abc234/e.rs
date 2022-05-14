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
        X:i64,
    }
    let xs = X.to_string().chars().collect::<Vec<char>>();
    let len = xs.len();
    let head = xs[0].to_digit(10).unwrap() as i64;
    let mut ans = {
        let mut digit = 0;
        let mut ret = 0;
        {
            while digit < len {
                ret *= 10;
                ret += head + 1;
                digit += 1;
            }
        }
        ret
    };

    for i in 0..10 {
        let mut ret = head;
        let mut prev = head + i;
        for j in 0..len - 1 {
            ret *= 10;
            ret += prev;
            prev += i;
        }
        if prev > i + 9 || !valid(ret) {
            continue;
        }
        if ret >= X {
            ans = ans.min(ret)
        }
    }

    for i in -9..=0 {
        let mut ret = head;
        let mut prev = ret + i;
        for j in 0..len - 1 {
            ret *= 10;
            ret += prev;
            prev += i;
        }
        // println!("{}", ret);
        if prev < i || !valid(ret) {
            continue;
        }

        if ret >= X {
            ans = ans.min(ret);
        }
    }
    if head < 9 {
        for i in -9..=0 {
            let mut ret = head + 1;
            let mut prev = ret + i;
            for j in 0..len - 1 {
                ret *= 10;
                ret += prev;
                prev += i;
            }
            // println!("{}", ret);
            if prev < i || !valid(ret) {
                continue;
            }

            if ret >= X {
                ans = ans.min(ret);
            }
        }
    }
    println!("{}", ans);
}

fn valid(X: i64) -> bool {
    if X < 10 {
        return true;
    }
    let mut x = X;
    let mut prev = x % 10;
    x /= 10;
    let diff = x % 10 - prev;
    prev = x % 10;
    x /= 10;
    while x > 0 {
        if diff != x % 10 - prev {
            return false;
        }
        prev = x % 10;
        x /= 10;
    }
    return true;
}
