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
        S:String,
        Q:usize,
        QS: [[usize;3];Q],
    }
    let mut cs = S.chars().collect::<Vec<_>>();
    let mut cnt = 0;
    for q in 0..Q {
        if QS[q][0] == 1 {
            let a = QS[q][1] - 1;
            let b = QS[q][2] - 1;
            let mut c = a.min(b);
            let mut d = a.max(b);
            if cnt % 2 == 0 {
                let tmp = cs[a];
                cs[a] = cs[b];
                cs[b] = tmp;
            } else {
                if d < N {
                    c += N;
                    d += N;
                } else if N <= c {
                    c -= N;
                    d -= N
                } else {
                    c += N;
                    d -= N;
                }
                let tmp = cs[c];
                cs[c] = cs[d];
                cs[d] = tmp;
            }
        } else {
            cnt += 1;
        }
    }
    if cnt % 2 == 1 {
        for i in 0..N {
            let t = cs[i];
            cs[i] = cs[i + N];
            cs[i + N] = t;
        }
    }
    for c in cs {
        print!("{}", c);
    }
    println!("");
}
