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
        N:usize,K:i64,
        mut AB:[[i64;2];N],
    }
    AB.sort();
    // println!("{:?}", AB);
    let mut money = K;
    for i in 0..N {
        // println!("{} {}", ans, money);
        if AB[i][0] > money {
            break;
        }
        // ans += AB[i][0] as i64;
        money += AB[i][1] as i64;
    }
    println!("{}", money);
}
