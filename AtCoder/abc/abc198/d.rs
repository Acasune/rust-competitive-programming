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
        A:String,
        B:String,
        C:String,
    }
    let a = A.chars().collect::<Vec<char>>();
    let b = B.chars().collect::<Vec<char>>();
    let c = C.chars().collect::<Vec<char>>();
    let mut g_vec = a
        .iter()
        .chain(b.iter())
        .chain(c.iter())
        .cloned()
        .collect::<HashSet<char>>()
        .iter()
        .cloned()
        .collect::<Vec<char>>();
    g_vec.sort();

    let g_len = g_vec.len();
    if g_len > 10 {
        println!("{}", "UNSOLVABLE");
        return;
    }
    let mut used = vec!['_'; 10];

    if !dfs(0, g_len, &mut g_vec, &mut used, &a, &b, &c) {
        println!("{}", "UNSOLVABLE");
    }
}

fn check(used: &mut Vec<char>, a: &Vec<char>, b: &Vec<char>, c: &Vec<char>) -> bool {
    let mut mapping = HashMap::<char, i64>::new();
    for i in 0..used.len() {
        if used[i] != '_' {
            mapping.insert(used[i], i as i64);
        }
    }
    if *mapping.get(&a[0]).unwrap() == 0
        || *mapping.get(&b[0]).unwrap() == 0
        || *mapping.get(&c[0]).unwrap() == 0
    {
        return false;
    }
    let mut an = 0;
    let mut bn = 0;
    let mut cn = 0;
    for i in 0..a.len() {
        an = an * 10 + *mapping.get(&a[i]).unwrap();
    }
    for i in 0..b.len() {
        bn = bn * 10 + *mapping.get(&b[i]).unwrap();
    }
    for i in 0..c.len() {
        cn = cn * 10 + *mapping.get(&c[i]).unwrap();
    }

    if an + bn == cn {
        println!("{}", an);
        println!("{}", bn);
        println!("{}", cn);
        return true;
    }
    return false;
}

fn dfs(
    n: usize,
    N: usize,
    chars: &mut Vec<char>,
    used: &mut Vec<char>,
    a: &Vec<char>,
    b: &Vec<char>,
    c: &Vec<char>,
) -> bool {
    if n == N {
        return check(used, a, b, c);
    }
    for i in 0..10 {
        if used[i] == '_' {
            used[i] = chars[n];
            if dfs(n + 1, N, chars, used, a, b, c) {
                return true;
            }
            used[i] = '_';
        }
    }

    return false;
}
