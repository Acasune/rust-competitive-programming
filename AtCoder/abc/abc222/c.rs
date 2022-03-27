#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Binary;
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize, M:usize,
        A:[Chars;2*N],
    }
    let mut memo = vec![0i64; 2 * N];
    for m in 0..M {
        let mut heap = BinaryHeap::<(i64, Reverse<usize>)>::new();
        for i in 0..2 * N {
            heap.push((memo[i], Reverse(i)));
        }
        while !heap.is_empty() {
            let (a_w, Reverse(a)) = heap.pop().unwrap();
            let (b_w, Reverse(b)) = heap.pop().unwrap();

            let result = result(A[a][m], A[b][m]);
            memo[a] += result.max(0);
            memo[b] += (-result).max(0);
        }
    }

    let mut ans = BinaryHeap::<(i64, Reverse<usize>)>::new();
    for i in 0..2 * N {
        ans.push((memo[i], Reverse(i + 1)));
    }
    while let Some((_, Reverse(i))) = ans.pop() {
        println!("{}", i);
    }
}

fn result(ca: char, cb: char) -> i64 {
    if ca == 'G' {
        if cb == 'C' {
            return 1;
        } else if cb == 'G' {
            return 0;
        } else {
            return -1;
        }
    } else if ca == 'C' {
        if cb == 'G' {
            return -1;
        } else if cb == 'C' {
            return 0;
        } else {
            return 1;
        }
    } else {
        if cb == 'G' {
            return 1;
        } else if cb == 'C' {
            return -1;
        } else {
            return 0;
        }
    }
}
