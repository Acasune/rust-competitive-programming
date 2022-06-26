#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
        N:usize,
        C:[usize;9]
    }
    let mn = C.iter().cloned().min().unwrap();
    let mut mn_idx = 0;
    for i in (0..9).rev() {
        if mn == C[i] {
            mn_idx = i;
            break;
        }
    }
    let keta = N/mn;
    let mut ans = vec![mn_idx +1;keta];
    let mut remain = N%mn;
    let mut now = 0;
    // println!("{}",remain);
    while remain > 0 {
        let mut flg =false;
        for i in (0..9).rev() {
            if i == mn_idx {
                break;
            }
            if remain + mn >= C[i] {
                ans[now] = i+1;
                remain -= C[i] - mn;
                flg = true;
                break;
            }
        }
        if !flg {
            break;
        }
        now +=1;
    }
    println!("{}",ans.into_iter().map(|i| i.to_string()).collect::<String>());

}
