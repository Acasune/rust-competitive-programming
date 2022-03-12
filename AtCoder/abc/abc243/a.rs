#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        mut V:i64,A:i64,B:i64,C:i64,
    }
    loop {
        if V < A {
            println!("{}", "F");
            return;
        }
        V -= A;
        if V < B {
            println!("{}", "M");
            return;
        }
        V -= B;
        if V < C {
            println!("{}", "T");
            return;
        }
        V -= C;
    }
}
