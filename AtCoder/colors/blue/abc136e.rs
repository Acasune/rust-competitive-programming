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
        N:usize,K:usize,
        A:[usize;N]
    }

    let mut ans =1;
    let mut divs = vec![];
    let total = A.iter().sum::<usize>();
    let mut i = 1;
    while i * i<=total {
        if total%i == 0 {
            divs.push(i);
            if i * i != total {
                divs.push(total/i);
            }
        }
        i +=1;
    }
    divs.sort();
    for di in divs {
        let mut arr = vec![0];
        for &a in &A {
            arr.push(a%di);
        }
        arr.sort();
        let mut rev = arr.clone();
        for i in 0..N {
            arr[i+1] += arr[i];
            rev[i+1] = di - rev[i+1] + rev[i];
        }
        for i in 0..=N {
            let b = rev[N] - rev[i];
            let c = arr[i];
            let gre = b.max(c);
            if gre <= K {
                ans = di;
            }
        }
    }
    println!("{}",ans);

}
