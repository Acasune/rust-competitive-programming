#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

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
        N:usize,
        K:usize,
        mut XY:[(i64,i64);N]
    }
    if K == 1 {
        println!("{}", "Infinity");
        return;
    }
    XY.sort();
    let mut mp = HashMap::<(i64, i64), Vec<usize>>::new();
    let mut mp2 = HashMap::<((i64, i64), usize), Vec<usize>>::new();
    for i in 0..N {
        for j in i + 1..N {
            let x0 = XY[i].0;
            let y0 = XY[i].1;
            let x1 = XY[j].0;
            let y1 = XY[j].1;
            let (a, b, g) = ext_gcd((y0 - y1), (x0 - x1));
            let tilt = ((y0 - y1) / g, (x0 - x1) / g);
            let mut idx_org = inf as usize;

            if let Some(vec) = mp.get(&tilt) {
                let mut flg = false;
                for &idx in vec {
                    let x2 = XY[idx].0;
                    let y2 = XY[idx].1;
                    if (x0 - x1) * (y2 - y1) == (y0 - y1) * (x2 - x1) {
                        mp2.get_mut(&(tilt, idx)).unwrap().push(i);
                        idx_org = idx;
                        break;
                    }
                }
            } else {
                mp.insert(tilt, vec![]);
            }
            if idx_org == inf as usize {
                mp.get_mut(&tilt).unwrap().push(i);
                mp2.insert((tilt, i), vec![i, j]);
            } else {
                mp2.get_mut(&(tilt, idx_org)).unwrap().push(i);
                mp2.get_mut(&(tilt, idx_org)).unwrap().push(j);
            }
        }
    }
    let mut ans = 0i64;
    for (k, v) in mp2 {
        let st = v.into_iter().collect::<HashSet<_>>();
        if st.len() >= K {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - b / a * x, x, g)
    }
}
