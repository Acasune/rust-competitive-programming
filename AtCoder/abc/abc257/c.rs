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
        S:Chars,
        W:[usize;N],
    }
    let mut l = 0;
    let mut children = vec![];
    let mut adults = vec![];
    let mut all = vec![inf_u];
    for (&c, &w) in S.iter().zip(W.iter()) {
        if c == '1' {
            adults.push(w);
        } else {
            children.push(w);
        }
        // all.push(w-1);
        all.push(w);
        // all.push(w+1);
    }
    adults.sort();
    children.sort();
    all.sort();
    all.dedup();

    let mut ai  = 0;
    let mut ci = 0;
    let mut all_i = 0;
    let mut ans = 0;
    let an = adults.len();
    let cn = children.len();
    let all_n = all.len();
    while all_i < all_n  {
        let w = all[all_i];
        while ai < an && adults[ai] < w {
            ai+=1;
        }
        while ci < cn && children[ci] < w {
            ci+=1;
        }
        let tmp = an - ai + ci;
        // println!("{} {} {} {}",ai,ci,w,tmp);
        ans = ans.max(tmp);
        all_i+=1;

    }
    println!("{}",ans);


}
