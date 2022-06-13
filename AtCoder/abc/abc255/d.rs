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
        N:usize,Q:usize,
        mut A:[usize;N],
    }
    A.reverse();
    A.push(0);
    A.reverse();
    A.push(inf_u);
    A.sort();
   
    let mut cum:Vec<usize> = vec![A[0]];
    for i in 1..N+1 {
        cum.push(cum[i-1]+A[i]);
    }

    for _ in 0..Q {
        input! {x:usize}
        let idx = A
        .binary_search_by_key(&(2 * (x+1)), |b| 2 * b + 1)
        .err()
        .unwrap() ;
        let mut ans:usize = (idx-1)*x - cum[idx-1] ;
        ans +=cum[N]- cum[idx-1] -(N+1-idx)*x;
        println!("{}",ans);
    }
}
