#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: usize = usize::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize, M:usize,
    }
    let mut queues = vec![VecDeque::<usize>::new(); M];
    let mut cols = vec![vec![]; N];
    let mut que = VecDeque::new();

    for m in 0..M {
        input! {
          K:usize,
          A:[Usize1;K],
        }
        for i in 0..K {
            queues[m].push_back(A[i]);
        }
        cols[queues[m].pop_front().unwrap()].push(m);
    }
    for i in 0..N {
        if cols[i].len() == 2 {
            que.push_back(i);
        }
    }

    while let Some(col) = que.pop_front() {
        for i in 0..cols[col].len() as usize {
            let m = cols[col][i];
            if let Some(nxt) = queues[m].pop_front() {
                cols[nxt].push(m);
                if cols[nxt].len() == 2 {
                    que.push_back(nxt);
                }
            }
        }
    }
    for q in queues {
        if !q.is_empty() {
            println!("{}", "No");
            return;
        }
    }

    println!("{}", "Yes");
}
