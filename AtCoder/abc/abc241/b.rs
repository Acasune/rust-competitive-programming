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
        N:usize, M:usize,
        A:[usize;N],
        B:[usize;M],
    }

    let mut map = HashMap::<usize, usize>::new();
    for &a in &A {
        if !map.contains_key(&a) {
            map.insert(a, 0);
        }
        *map.get_mut(&a).unwrap() += 1;
    }
    for &b in &B {
        if !map.contains_key(&b) || *map.get(&b).unwrap() == 0 {
            println!("{}", "No");
            return;
        }
        *map.get_mut(&b).unwrap() -= 1;
    }
    println!("{}", "Yes");
}
