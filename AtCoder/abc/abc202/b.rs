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
       S:String,
    }
    let mut cl = S.chars().collect::<Vec<char>>();
    cl.reverse();
    for c in cl {
        if c == '6' {
            print!("{}", '9');
        } else if c == '9' {
            print!("{}", '6');
        } else {
            print!("{}", c);
        }
    }
    println!("");
}
