#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,Q:usize,

    }
    let mut pre = vec![inf as usize; N];
    let mut post = vec![inf as usize; N];
    for _ in 0..Q {
        input! {
            ask :usize,
        }
        if ask == 1 {
            input! {
               x:Usize1,
               y:Usize1,
            }
            post[x] = y;
            pre[y] = x;
        }
        if ask == 2 {
            input! {
               x:Usize1,
               y:Usize1,
            }
            post[x] = inf as usize;
            pre[y] = inf as usize;
        }
        if ask == 3 {
            input! {
               x:Usize1,
            }
            let mut deq = VecDeque::<usize>::new();
            deq.push_back(x);
            let mut nxt = post[x];
            while nxt != inf as usize {
                deq.push_back(nxt);
                nxt = post[nxt];
            }
            let mut nxt = pre[x];
            while nxt != inf as usize {
                deq.push_front(nxt);
                nxt = pre[nxt];
            }
            print!("{}", deq.len());
            deq.into_iter().for_each(|e| print!(" {}", e + 1));
            println!("");
        }
    }
}
