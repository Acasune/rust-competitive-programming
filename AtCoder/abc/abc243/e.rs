#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::{BinaryHeap, HashSet};

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize, M:usize,
        ABC:[(usize, usize, usize);M],
    }
    let inf = i64::MAX / 10;
    let mut vexes = vec![vec![inf; N]; N];

    let mut st = HashSet::<(usize, usize)>::new();
    for &(a, b, c) in &ABC {
        vexes[a - 1][b - 1] = c as i64;
        vexes[b - 1][a - 1] = c as i64;
        st.insert((a - 1, b - 1));
    }
    let mut vexes2 = vec![vec![inf; N]; N];
    for i in 0..N {
        vexes2[i][i] = 0;
    }
    let mut st = HashSet::<(usize, usize)>::new();
    for &(a, b, c) in &ABC {
        vexes2[a - 1][b - 1] = c as i64;
        vexes2[b - 1][a - 1] = c as i64;
        st.insert((a - 1, b - 1));
    }
    let mut flgs = vec![vec![false; N]; N];

    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                if vexes[i][j] >= vexes[i][k] + vexes[k][j] {
                    let x = i.min(j);
                    let y = i.max(j);
                    st.remove(&(x, y));
                    vexes[i][j] = vexes[i][k] + vexes[k][j];
                }
            }
        }
    }
    println!("{}", M - st.len());
}
