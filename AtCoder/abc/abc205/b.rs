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
        N:usize,
        mut V:[usize;N],
    }
    V.sort();
    // println!("{:?}", V);
    for i in 1..=N {
        if V[i - 1] != i {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
