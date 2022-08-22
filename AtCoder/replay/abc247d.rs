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
        Q:usize,
    }
    let mut que = VecDeque::new();
    for _ in 0..Q {
        input! {
            ask:usize
        }
        if ask == 1 {
            input! {
                x:i64,
                c:i64,
            }
            que.push_back((x, c));
        } else {
            input! {
                mut c:i64,
            }
            let mut ans = 0;
            while c > 0 {
                let (x, d) = que.pop_front().unwrap();
                if c >= d {
                    ans += x * d;
                    c -= d;
                } else {
                    let e = d - c;
                    ans += x * c;
                    que.push_front((x, e));
                    break;
                }
            }
            println!("{}", ans);
        }
    }
}
