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
       mut X:i128, A:i128,mut D:i128, N:i128
    }
    if D == 0 {
        println!("{}",(A-X).abs());
        return;
    }
    X -=A;
    let mut ans = 0;
    if D<0 {
        if X>=0 {
            ans = X;
        }else {
            D = D.abs();
            X = X.abs();
            let last = D*(N-1);
            let a= X%D;
            let tmp = X-last;
            if tmp >=0 {
                ans = tmp;
            } else {
                ans = (D-a).min(a);
            }
        }
    } else {
        // D>0
        if X <=0 {
            ans = X.abs();
        }else {
            let last = D*(N-1);
            let a= X%D;
            let tmp = X-last;
            if tmp >=0{
                ans = tmp;
            } else {
                ans = (D-a).min(a);
            }

        }
    }
    println!("{}",ans);

}
