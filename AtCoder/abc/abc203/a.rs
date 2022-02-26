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
        A:i64, B:i64, C:i64,
    }

    if A == B {
        println!("{}", C);
    } else if C == B {
        println!("{}", A);
    } else if A == C {
        println!("{}", B);
    } else {
        println!("{}", 0);
    }
}
