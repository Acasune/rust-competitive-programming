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
        ST: [(String,usize);N],
    }
    let mut vc = Vec::<(String, usize)>::from(ST);
    vc.sort_by_key((|a| a.1));
    println!("{}", vc[N - 2].0);
}
