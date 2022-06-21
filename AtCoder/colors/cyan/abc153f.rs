#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, BTreeSet, VecDeque};
use std::convert::TryInto;
use std::{char,i32,f32,f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,D:usize, A:usize,
        mut XH:[(usize,usize);N],
    }
    let mut ans = 0;
    XH.sort();
    let mut que = VecDeque::new();
    let mut now = 0;
    for (x,h) in XH {
        while let Some((y,z)) = que.pop_front() {
            if y < x {
                now -= z;
            } else {
                que.push_front((y,z));
                break;
            }
        }
        if h <= now {
            continue;
        }
        let cnt = (h- now + A-1)/A;
        now += cnt * A;
        ans +=cnt;
        que.push_back((x + 2*D, cnt * A));
    }
    println!("{}",ans);


}
