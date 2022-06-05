#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,
        chars:[Chars;N],
    }
    let mut ans = inf_u;
    for i in 0..10 {
        let c = char::from_digit(i, 10).unwrap();
        let mut roll = 0;
        let mut memo = vec![false; N];
        let mut cnt = 0;
        while cnt < N {
            let mut idx = roll % 10;
            // println!("{}", cnt);
            for i in 0..N {
                if chars[i][idx] == c && !memo[i] {
                    memo[i] = true;
                    cnt += 1;
                    break;
                }
            }
            roll += 1;
        }
        ans = ans.min(roll - 1);
        // println!("{} {}", c, roll);
    }
    println!("{}", ans);
}
