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

fn main() {
    input! {
        N:usize,
        A:[usize;N],
        B:[usize;N],
        Q:usize,
        ask:[(Usize1,Usize1);Q],
    }

    let mut ans = vec![vec![inf_u, inf_u]; N];
    let mut b_i = 0;
    let mut st_a = HashSet::new();
    let mut st_b = HashSet::new();
    let mut st_ab = HashSet::new();
    for a_i in 0..N {
        let a = A[a_i];
        if st_ab.contains(&a) {
            ans[a_i] = ans[a_i - 1].clone();
        } else {
            if st_b.contains(&a) {
                st_ab.insert(a);
                st_b.remove(&a);
            } else {
                st_a.insert(a);
            }
            while b_i < N && st_a.len() != 0 && st_b.len() == 0 {
                let b = B[b_i];
                if !st_ab.contains(&b) {
                    if st_a.contains(&b) {
                        st_ab.insert(b);
                        st_a.remove(&b);
                    } else {
                        st_b.insert(b);
                    }
                }
                b_i += 1;
            }
            if st_a.len() > 0 {
                ans[a_i] = vec![inf_u, inf_u];
            } else {
                let l = b_i - 1;
                while b_i < N && st_ab.contains(&B[b_i]) {
                    b_i += 1;
                }
                ans[a_i] = vec![l, b_i];
            }
        }
    }
    for q in ask {
        let a = q.0;
        let b = q.1;
        if ans[a][0] <= b && b < ans[a][1] {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
