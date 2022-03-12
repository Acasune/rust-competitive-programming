#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        L:i64, R:i64,
    }
    let mut ans = 0i64;
    let mut cnts = vec![0; R as usize + 10];
    for g in (2..R + 10).rev() {
        cnts[g as usize] = (R / g - (L - 1) / g) * (R / g - (L - 1) / g);
        let mut g_i = g * 2;
        while g_i <= R {
            cnts[g as usize] -= cnts[g_i as usize];
            g_i += g;
        }
    }
    for g in 2..=R {
        ans += cnts[g as usize];
        if L <= g && g <= R {
            ans -= R / g - (L - 1) / g;
            ans -= R / g - (L - 1) / g;
            ans += 1;
        }
    }
    println!("{}", ans);
}
