#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::sync::Arc;
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
        A:[i64;N],
    }
    let len: usize = N.min(9);
    let mut mp = HashMap::<i64, Vec<usize>>::new();
    for i in 1..1 << len {
        let mut al = Vec::<usize>::new();
        let mut a = 0i64;
        for j in 0..len {
            if (i >> j) & 1 == 1 {
                a += A[j];
                al.push(j);
            }
        }
        a %= 200;
        if mp.contains_key(&a) {
            println!("{}", "Yes");
            print!("{}", al.len());
            for &b in &al {
                print!(" {}", b + 1);
            }
            println!("");
            print!("{}", mp.get(&a).unwrap().len());
            for c in mp.get(&a).unwrap() {
                print!(" {}", c + 1);
            }
            println!("");
            return;
        }
        mp.insert(a, al.clone());
    }
    println!("{}", "No");
}
