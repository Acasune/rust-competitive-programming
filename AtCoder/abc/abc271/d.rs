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
        N:usize,S:usize,
        ab:[(usize,usize);N]
    }
    let mut visited = vec![vec![vec![(inf_u, inf_u, inf_u); 2]; S + 1]; N + 1];
    visited[0][0][0] = (10000, 0, 0);
    visited[0][0][1] = (10000, 0, 0);
    let cent = (inf_u, inf_u, inf_u);

    for i in 0..N {
        let a = ab[i].0;
        let b = ab[i].1;
        for s in 0..S {
            for j in 0..2 {
                if visited[i][s][j] != cent && s + a <= S {
                    visited[i + 1][s + a][0] = (i, s, j);
                }
                if visited[i][s][j] != cent && s + b <= S {
                    visited[i + 1][s + b][1] = (i, s, j);
                }
            }
        }
    }
    if visited[N][S][0] == cent && visited[N][S][1] == cent {
        println!("{}", "No");
        return;
    }
    println!("{}", "Yes");
    let mut ans = vec![];
    let mut state = (N, S, 0);
    if visited[N][S][0] == cent {
        state = (N, S, 1);
    }
    while visited[state.0][state.1][state.2] != (10000, 0, 0) {
        if state.2 == 0 {
            ans.push('H');
        } else {
            ans.push('T')
        }
        state = visited[state.0][state.1][state.2];
    }
    ans.reverse();
    let SS = ans.into_iter().collect::<String>();
    println!("{}", SS);
}
