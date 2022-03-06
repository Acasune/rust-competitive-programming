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
        a:i64, b:i64, c:f64, x:i64,
    }
    if x <= a {
        println!("{:.10}", 1.);
        return;
    } else if x <= b {
        println!("{:.10}", c / ((b - a) as f64));
    } else {
        println!("{:.10}", 0.0);
    }
}
