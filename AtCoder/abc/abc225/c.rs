#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,M:usize,
        B:[[Usize1;M];N],
    }
    for i in 1..N {
        for j in 0..M {
            if B[i][j] != B[i - 1][j] + 7 {
                println!("{}", "No");
                return;
            }
        }
    }
    for i in 0..N {
        for j in 1..M {
            if B[i][j] != B[i][j - 1] + 1 {
                println!("{}", "No");
                return;
            }
        }
    }
    let l_x = B[0][0] % 7;
    let r_x = B[0][M - 1] % 7;
    if l_x > r_x {
        println!("{}", "No");
        return;
    }
    println!("{}", "Yes");
}
