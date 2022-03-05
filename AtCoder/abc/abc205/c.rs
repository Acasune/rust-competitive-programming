#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

fn main() {
    input! {
        A:i64,B:i64,C:i64,
    }
    let isEven = C % 2 == 0;
    if isEven {
        let a_abs = A.abs();
        let b_abs = B.abs();
        if a_abs < b_abs {
            println!("<");
        } else if a_abs == b_abs {
            println!("=");
        } else {
            println!(">");
        }
    } else {
        if A < B {
            println!("<");
        } else if A == B {
            println!("=");
        } else {
            println!(">");
        }
    }
}
