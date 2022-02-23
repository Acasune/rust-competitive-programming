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
    let cl = S.chars().collect::<Vec<char>>();
    let mut a = HashSet::new();
    let mut b = HashSet::new();
    for i in 0..cl.len() {
        if cl[i] == 'o' {
            a.insert(i);
            b.insert(i);
        }
        if cl[i] == '?' {
            b.insert(i);
        }
    }

    let mut ans = 0i64;
    for i in 0..10000 {
        let mut c = HashSet::new();
        let mut tmp = i;
        for _ in 0..4 {
            c.insert(tmp % 10);
            tmp /= 10
        }

        // println!("{:?}", c);
        let mut flg = true;
        for &aa in &a {
            if !c.contains(&aa) {
                flg = false;
                break;
            }
        }
        for &cc in &c {
            if !b.contains(&cc) {
                flg = false;
                break;
            }
        }
        if flg {
            // println!("{}", i);
            ans += 1;
        }
    }
    println!("{}", ans);
}
