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

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,X:usize,Y:usize,
        mut A:[usize;N],
    }
    A.push(inf_u);
    let mut i = 0;
    let mut ans = 0;
    let mut B = vec![];
    while i < N + 1 {
        if Y <= A[i] && A[i] <= X {
            B.push(A[i]);
        } else {
            if B.len() != 0 {
                ans += calc(B, X, Y);
            }
            B = vec![];
        }
        i += 1;
    }
    println!("{}", ans);
}

fn calc(B: Vec<usize>, L: usize, R: usize) -> usize {
    // println!("{:?}", B);
    let mut sum = 0;
    let n = B.len();
    let mut l = 0;
    let mut r = 0;
    let mut visited = vec![0, 0];
    while r < n {
        while r < n {
            if !(visited[0] > 0 && visited[1] > 0) {
                if B[r] == L {
                    visited[0] += 1;
                }
                if B[r] == R {
                    visited[1] += 1;
                }
            } else {
                break;
            }
            r += 1;
        }
        while l <= r {
            if !(visited[0] > 0 && visited[1] > 0) {
                break;
            } else {
                sum += n - r + 1;
                if B[l] == L {
                    visited[0] -= 1;
                }
                if B[l] == R {
                    visited[1] -= 1;
                }
                l += 1;
            }
        }
    }

    sum
}
