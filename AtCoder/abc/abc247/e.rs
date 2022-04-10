#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
     N:usize,X:usize,Y:usize,
     A:[usize;N]
    }
    let mut l = 0;
    let mut r = 0;
    let mut ans = 0usize;
    while r < N {
        let mut v = vec![];
        while r < N && Y <= A[r] && A[r] <= X {
            v.push(A[r]);
            r += 1;
        }
        ans += calc(v, X, Y);
        r += 1;
    }

    println!("{}", ans);
}

fn calc(vec: Vec<usize>, X: usize, Y: usize) -> usize {
    let mut r = 0;
    let mut l = 0;
    let mut cnt_x = 0;
    let mut cnt_y = 0;
    let n = vec.len();
    let mut ret = 0;
    while l < n {
        while r < n && (cnt_x == 0 || cnt_y == 0) {
            if vec[r] == X {
                cnt_x += 1;
            }
            if vec[r] == Y {
                cnt_y += 1;
            }
            r += 1;
        }
        if cnt_x > 0 && cnt_y > 0 {
            ret += n - r + 1;
        }
        if vec[l] == X {
            cnt_x -= 1;
        }
        if vec[l] == Y {
            cnt_y -= 1;
        }
        l += 1;
    }
    return ret;
}
