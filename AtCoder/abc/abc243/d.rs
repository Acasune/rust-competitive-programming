#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize,X:i128,
        S:Chars
    }
    let mut stk = vec![];
    for i in 0..N {
        let c = S[i];
        let len = stk.len();
        if c == 'U' {
            if len == 0 || stk[len - 1] == 'U' {
                stk.push('U');
            } else {
                // len >0 && stk.last != U
                stk.pop();
            }
        } else {
            stk.push(c);
        }
    }
    let mut ans = X;
    for i in 0..stk.len() {
        if stk[i] == 'U' {
            ans = ans / 2;
        } else if stk[i] == 'L' {
            ans = ans * 2;
        } else {
            ans = ans * 2 + 1;
        }
        // println!("{}", ans);
    }
    println!("{}", ans);
}
