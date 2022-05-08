#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,
        AB:[(Usize1,Usize1);M],
        CD:[(Usize1,Usize1);M],
    }
    let mut t_vec = vec![vec![]; N];
    let mut a_vec = vec![vec![]; N];
    for (a, b) in AB {
        t_vec[a].push(b);
        t_vec[b].push(a);
    }
    for (a, b) in CD {
        a_vec[a].push(b);
        a_vec[b].push(a);
    }
    for perm in (0..N).permutations(N) {
        // println!("{:?}", perm);
        let mut flg = true;
        for i in 0..N {
            // check takahashi i == aoki perm[i]
            for &te in &t_vec[i] {
                let mut inner_flg = false;
                for &ae in &a_vec[perm[i]] {
                    if perm[te] == ae {
                        inner_flg = true;
                        break;
                    }
                }
                if !inner_flg {
                    flg = false;
                    break;
                }
            }
        }
        if flg {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
