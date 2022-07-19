#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Ordering::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
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
        N:usize,X:usize,Y:usize,Z:usize,
        A:[usize;N],
        B:[usize;N]
    }
    let mut ans = vec![];
    let mut students = vec![];
    for (i, (a, b)) in A.into_iter().zip(B.into_iter()).enumerate() {
        students.push((a, b, i + 1));
    }
    students.sort_by_key(|&(a, _, i)| (Reverse(a), i));
    students[X..].sort_by_key(|&(_, b, i)| (Reverse(b), i));
    students[(X + Y)..].sort_by_key(|&(a, b, i)| (Reverse(a + b), i));

    for i in 0..(X + Y + Z) {
        ans.push(students[i].2);
    }
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}
