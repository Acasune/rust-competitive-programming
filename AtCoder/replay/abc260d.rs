#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

type Circle = (i64, i64, i64);

fn main() {
    input! {
        N:usize,K:usize,
        P:[i64;N],
    }
    let mut st = BTreeSet::new();
    let mut cds = vec![vec![]; N];
    let mut ans = vec![-1; N];
    let mut zeros = vec![];
    for i in 0..N {
        zeros.push(i);
    }

    for i in 0..N {
        let p = P[i];
        if let Some(&(q, j, idx)) = st.range((p, 0, 0)..).next() {
            st.remove(&(q, j, idx));
            cds[j].push((p, i));
            if cds[j].len() == K {
                for &v in &cds[j] {
                    ans[v.0 as usize - 1] = i as i64 + 1;
                }
                cds[j] = vec![];
                zeros.push(j);
            } else {
                st.insert((p, j, i));
            }
        } else {
            let j = zeros.pop().unwrap();
            cds[j].push((p, i));
            if cds[j].len() == K {
                for &v in &cds[j] {
                    ans[v.0 as usize - 1] = i as i64 + 1;
                }
                cds[j] = vec![];
                zeros.push(j);
            } else {
                st.insert((p, j, i));
            }
            st.insert((p, j, i));
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
