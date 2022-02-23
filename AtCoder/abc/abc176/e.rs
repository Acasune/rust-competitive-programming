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
        H:usize, W:usize, N:usize,
        hw:[[i64;2];N],
    }
    let mut mp1 = HashMap::<i64, i64>::new();
    let mut mp2 = HashMap::<i64, i64>::new();
    let mut st = HashSet::<(i64, i64)>::new();
    let mut hmax = 0i64;
    let mut wmax = 0i64;

    for i in 0..N {
        let h = hw[i][0];
        let w = hw[i][1];

        match mp1.get(&h) {
            Some(&a) => {
                *mp1.get_mut(&h).unwrap() += 1;
            }
            None => {
                mp1.insert(h, 1);
            }
        }

        match mp2.get(&w) {
            Some(a) => {
                *mp2.get_mut(&w).unwrap() += 1;
            }
            None => {
                mp2.insert(w, 1);
            }
        }
        hmax = hmax.max(*mp1.get(&h).unwrap());
        wmax = wmax.max(*mp2.get(&w).unwrap());
        st.insert((h, w));
    }
    let mut v1_vec = Vec::<i64>::new();
    let mut v2_vec = Vec::<i64>::new();
    for (&a, &b) in &mp1 {
        if b == hmax {
            v1_vec.push(a);
        }
    }
    for (&a, &b) in &mp2 {
        if b == wmax {
            v2_vec.push(a);
        }
    }

    let mut ans = wmax + hmax - 1;
    for &e1 in &v1_vec {
        for &e2 in &v2_vec {
            if !st.contains(&(e1, e2)) {
                println!("{}", ans + 1);
                return;
            }
        }
    }

    println!("{}", ans);
}
