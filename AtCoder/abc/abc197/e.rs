#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::BinaryHeap;

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize,
        XT:[(i64,usize);N],
    }
    let inf = i64::MAX / 10;
    let mut colors = vec![(inf, -inf); N + 1];

    for &(x, c) in &XT {
        colors[c].0 = x.min(colors[c].0);
        colors[c].1 = x.max(colors[c].1);
    }
    colors[0].0 = 0;
    colors[0].1 = 0;
    colors = colors
        .into_iter()
        .filter(|&(l, r)| l != inf && r != inf)
        .collect::<Vec<(i64, i64)>>();
    colors.push((0, 0));

    let n_c = colors.len();
    let mut memo = vec![vec![inf, inf]; n_c + 1];
    memo[0][0] = 0;
    memo[0][1] = 0;
    for i in 1..n_c {
        // next standpoint is left ,previous standpoint is left
        memo[i][0] = memo[i][0].min(
            memo[i - 1][0]
                + (colors[i].1 - colors[i - 1].0).abs()
                + (colors[i].0 - colors[i].1).abs(),
        );
        // next standpoint is left ,previous standpoint is right
        memo[i][0] = memo[i][0].min(
            memo[i - 1][1]
                + (colors[i].1 - colors[i - 1].1).abs()
                + (colors[i].0 - colors[i].1).abs(),
        );
        // next standpoint is right ,previous standpoint is left
        memo[i][1] = memo[i][1].min(
            memo[i - 1][0]
                + (colors[i].0 - colors[i - 1].0).abs()
                + (colors[i].1 - colors[i].0).abs(),
        );
        // next standpoint is left ,previous standpoint is right
        memo[i][1] = memo[i][1].min(
            memo[i - 1][1]
                + (colors[i].0 - colors[i - 1].1).abs()
                + (colors[i].1 - colors[i].0).abs(),
        );
    }
    println!("{}", memo[n_c - 1][0].min(memo[n_c - 1][1]));
}
