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
        mut X:Chars,
    }
    let N = X.len();
    let mut Y = X
        .iter()
        .scan(0, |cum, x| {
            *cum += (*x as u8 - '0' as u8) as i64;
            Some(*cum)
        })
        .collect::<Vec<i64>>();
    Y.reverse();
    let mut ans = vec![];
    let mut next = 0;
    let mut base_10 = 1;
    for i in 0..N {
        let mut base = next;
        base += Y[i];

        ans.push(base % 10);
        next = base / 10;
    }
    while next > 0 {
        ans.push(next % 10);
        next /= 10;
    }
    ans.reverse();
    for a in ans {
        print!("{}", a);
    }
    println!();
}
