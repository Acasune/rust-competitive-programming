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
        H:usize, W:usize,
        S:[String;H],
    }
    let mut maze = vec![vec![0i64; W]; H];
    for h in 0..H {
        let mut w = 0;
        for c in S[h].chars() {
            if c == '+' {
                maze[h][w] = 1;
            } else {
                maze[h][w] = -1;
            }
            w += 1;
        }
    }
    if H == 1 && W == 1 {
        println!("{}", "Draw");
        return;
    }
    let mut points = vec![vec![(0i64, 0i64); W]; H];
    let isTakahashiLeft = ((H - 1) + (W - 1)) % 2 == 1;

    for i in (0..(H + W - 2)).rev() {
        let mut w = 0;
        let mut h = 0;
        let mut j = H;
        if i >= H {
            w = i - H + 1;
            h = H - 1;
            j = H.min(W - w);
        } else {
            w = 0;
            h = i;
            j = (h + 1).min(W - w);
        }

        for k in 0..j {
            let nw = w + k;
            let nh = h - k;
            if nw == W - 1 {
                let (a, b) = points[nh + 1][nw];
                if i % 2 == 0 {
                    points[nh][nw] = (a, b + maze[nh + 1][nw]);
                } else {
                    points[nh][nw] = (a + maze[nh + 1][nw], b);
                }
            } else if nh == H - 1 {
                let (a, b) = points[nh][nw + 1];
                if i % 2 == 0 {
                    points[nh][nw] = (a, b + maze[nh][nw + 1]);
                } else {
                    points[nh][nw] = (a + maze[nh][nw + 1], b);
                }
            } else {
                let (a, b) = points[nh + 1][nw];
                let (c, d) = points[nh][nw + 1];
                if i % 2 == 0 {
                    let diff = b + maze[nh + 1][nw] - a >= d + maze[nh][nw + 1] - c;
                    if diff {
                        points[nh][nw] = (a, b + maze[nh + 1][nw]);
                    } else {
                        points[nh][nw] = (c, d + maze[nh][nw + 1]);
                    }
                } else {
                    let diff = a + maze[nh + 1][nw] - b >= c + maze[nh][nw + 1] - d;
                    if diff {
                        points[nh][nw] = (a + maze[nh + 1][nw], b);
                    } else {
                        points[nh][nw] = (c + maze[nh][nw + 1], d);
                    }
                }
            }
        }
    }
    if points[0][0].0 == points[0][0].1 {
        println!("{}", "Draw");
        return;
    }
    if points[0][0].0 < points[0][0].1 {
        println!("{}", "Takahashi");
    } else {
        println!("{}", "Aoki");
    }
}
