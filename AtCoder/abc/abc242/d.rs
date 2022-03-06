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
        Q:usize,
        TK:[(usize,usize);Q],
    }
    let mut chars = S.chars().collect::<Vec<char>>();
    for (mut t, mut k) in TK {
        let mut depth = 1usize;
        let mut cnt = 0;
        for i in 0..t {
            if depth > 1_000_000_000_000_000_000 {
                // cnt += 1;
                // depth *= 2;
                break;
            }
            cnt += 1;
            depth *= 2;
            // t -= 1;
        }
        let mut ans = 'z';

        if depth > 1_000_000_000_000_000_000 {
            let mut c = 'z';
            let idx = (k - 1) / depth;
            c = chars[idx];
            let o_c_i = (t - cnt) % 3;
            let memo = vec!['B', 'C', 'A'];
            let idx = {
                if c == 'A' {
                    0
                } else if c == 'B' {
                    1
                } else {
                    2
                }
            };
            c = memo[(idx + o_c_i + 2) % 3];
            ans = dps(c, k, 0, depth);
        // println!("{}", "eeee");
        } else {
            let mut c = 'z';
            let idx = (k - 1) / depth;
            c = chars[idx];
            ans = dps(c, k - idx * depth, 0, depth);
        }
        println!("{}", ans);
    }
}

fn dps(c: char, i: usize, l: usize, r: usize) -> char {
    if l == r - 1 {
        return c;
    }
    let memo = vec![('B', 'C'), ('C', 'A'), ('A', 'B')];
    let idx = {
        if c == 'A' {
            0
        } else if c == 'B' {
            1
        } else {
            2
        }
    };

    let m = (l + r) / 2;
    let (lc, rc) = memo[idx];
    if i <= m {
        return dps(lc, i, l, m);
    } else {
        return dps(rc, i, m, r);
    }
}
fn dps2(c: char, i: usize, t: usize, l: usize, r: usize) -> char {
    if l == r - 1 {
        return c;
    }
    let memo = vec![('B', 'C'), ('C', 'A'), ('A', 'B')];
    let idx = {
        if c == 'A' {
            0
        } else if c == 'B' {
            1
        } else {
            2
        }
    };

    let m = (l + r) / 2;
    let (lc, rc) = memo[idx];
    if i <= m {
        return dps(lc, i, l, m);
    } else {
        return dps(rc, i, m, r);
    }
}
