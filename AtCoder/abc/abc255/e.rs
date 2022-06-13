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
        N:usize,M:usize,
        S:[i64;N-1],
        X:[i64;M],
    }
    let mut B = vec![0];
    for i in 1..N {
        let mut b = S[i-1] - B[i-1];
        B.push(b);
    }
    let mut mp = HashMap::new();
    for i in 0..N {
        for j in 0..M {
            let mut c = X[j] - B[i];
            if i %2 == 1 {
                c *= -1;
            }
            *mp.entry(c).or_insert(0) +=1;
        }
    }
    let mut ans = 0;
    for (k,v) in mp {
        ans = ans.max(v);
    }
    println!("{}",ans);


}
