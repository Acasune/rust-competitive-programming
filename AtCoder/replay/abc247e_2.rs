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

fn main() {
    input! {
        N:usize,X:usize,Y:usize,
        mut A:[usize;N],
    }
    let mut lft = 0;
    let mut x = inf_u;
    let mut y = inf_u;
    let mut ans = 0;
    for i in 0..N {
        let a = A[i];
        if a > X || Y > a {
            x = inf_u;
            y = inf_u;
            lft = inf_u;
        } else {
            if lft == inf_u {
                lft = i;
            }
            if X == a {
                x = i;
            }
            if Y == a {
                y = i;
            }
            if x != inf_u && y != inf_u {
                let mn = x.min(y);
                let mx = x.max(y);
                ans += mn - lft + 1;
            }
        }
    }
    println!("{}", ans);
}
