#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

fn main() {
    input! {
        N:usize,
        XYZ:[[i64;3];N],
    }

    let mut bits: usize = 1 << N;
    let mut dp = vec![vec![1_000_000_000_000; N]; 1 << N];
    dp[1][0] = 0;

    // for j in 0..N {
    //     dp[1 << j][j] = XYZ[j][0].abs() + XYZ[j][1].abs() + 0.max(XYZ[j][2]);
    // }

    for bit in 1..bits {
        for j in 0..N {
            if bit >> j & 1 == 0 {
                for i in 0..N {
                    if (bit >> i) & 1 == 1 {
                        let tmp = (XYZ[j][0] - XYZ[i][0]).abs()
                            + (XYZ[j][1] - XYZ[i][1]).abs()
                            + 0.max(XYZ[j][2] - XYZ[i][2]);
                        dp[bit | (1 << j)][j] = dp[bit | (1 << j)][j].min(dp[bit][i] + tmp);
                    }
                }
            }
        }
    }
    let mut ans: i64 = 1_000_000_000_000;
    for i in 1..N {
        let tmp = (XYZ[0][0] - XYZ[i][0]).abs()
            + (XYZ[0][1] - XYZ[i][1]).abs()
            + 0.max(XYZ[0][2] - XYZ[i][2]);

        ans = ans.min(dp[(1 << N) - 1][i] + tmp);
    }
    println!("{}", ans);
}
