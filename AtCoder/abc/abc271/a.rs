#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = (i64::MAX / 10) * 9;
const inf_u: usize = (usize::MAX / 10) * 9;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        N:usize,
    }
    let mut mp = HashMap::new();
    mp.insert(0, '0');
    mp.insert(1, '1');
    mp.insert(2, '2');
    mp.insert(3, '3');
    mp.insert(4, '4');
    mp.insert(5, '5');
    mp.insert(6, '6');
    mp.insert(7, '7');
    mp.insert(8, '8');
    mp.insert(9, '9');
    mp.insert(10, 'A');
    mp.insert(11, 'B');
    mp.insert(12, 'C');
    mp.insert(13, 'D');
    mp.insert(14, 'E');
    mp.insert(15, 'F');

    let mut ans = vec!['a', 'a'];
    ans[0] = *mp.get(&(N / 16)).unwrap();
    ans[1] = *mp.get(&(N % 16)).unwrap();
    println!("{}", ans.into_iter().collect::<String>());
}
