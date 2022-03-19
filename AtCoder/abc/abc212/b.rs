#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        X:Chars
    }

    let mut XX = X.clone();
    if X.into_iter().unique().collect::<Vec<char>>().len() == 1 {
        println!("{}", "Weak");
        return;
    }
    for i in 0..XX.len() - 1 {
        if XX[i] == '9' {
            if XX[i + 1] != '0' {
                println!("{}", "Strong");
                return;
            }
        } else {
            let num = XX[i].to_digit(10).unwrap();
            let next = XX[i + 1].to_digit(10).unwrap();
            if num + 1 != next {
                println!("{}", "Strong");
                return;
            }
        }
    }
    println!("{}", "Weak");
}
