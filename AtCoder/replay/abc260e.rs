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

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,M:usize,
        AB:[(Usize1,Usize1);N]
    }
    let mut l = 0;
    let mut r = 0;
    let mut tree = RangeAddQueryTree::new(M + 1);
    let mut inv_l = vec![vec![]; M + 1];
    let mut inv_r = vec![vec![]; M + 1];
    let mut visited = vec![vec![false; 2]; M + 1];
    for i in 0..N {
        let (a, b) = AB[i];
        inv_l[a].push(i);
        inv_r[b].push(i);
    }
    let mut cnt = 0;
    while r < M {
        // println!("{} {} {}", l, r, cnt);
        while cnt != N && r < M {
            for &i in &inv_l[r] {
                visited[i][0] = true;
                cnt += 1;
            }
            for &i in &inv_r[r] {
                visited[i][1] = true;
                if !visited[i][0] {
                    cnt += 1;
                }
            }
            r += 1;
        }
        if cnt == N {
            tree.add(r - l, M - l + 1, 1);
        }

        while cnt == N && l < r {
            for &i in &inv_l[l] {
                visited[i][0] = false;
                if !visited[i][1] {
                    cnt -= 1;
                }
            }
            for &i in &inv_r[l] {
                visited[i][1] = false;
                if !visited[i][0] {
                    cnt -= 1;
                }
            }
            l += 1;
            if cnt == N {
                tree.add(r - l, M - l + 1, 1);
            }
        }
    }
    for i in 1..M + 1 {
        print!("{} ", tree.query(i, i + 1));
    }
    println!()
}

struct RangeAddQueryTree {
    n: usize,
    bits: Vec<Vec<i64>>,
}
// [l,r)
impl RangeAddQueryTree {
    fn new(n: usize) -> Self {
        let size = n;
        RangeAddQueryTree {
            n: size,
            bits: vec![vec![0; size]; 2],
        }
    }

    fn add_sub(&mut self, bit: usize, mut pos: usize, x: i64) {
        let mut cnt = 0;
        pos += 1;
        while pos <= self.n {
            self.bits[bit][pos - 1] += x;
            pos += pos & pos.wrapping_neg();
            cnt += 1;
        }
    }

    fn add(&mut self, l: usize, r: usize, x: i64) {
        self.add_sub(0, l, -x * (l as i64));
        self.add_sub(0, r, x * (r as i64));
        self.add_sub(1, l, x);
        self.add_sub(1, r, -x);
    }

    fn sum_sub(&self, bit: usize, mut pos: usize) -> i64 {
        let bits = &self.bits;
        let mut s = 0;
        while pos > 0 {
            s += bits[bit][pos - 1];
            pos -= pos & pos.wrapping_neg();
        }
        s
    }

    fn sum(&self, mut pos: usize) -> i64 {
        self.sum_sub(0, pos) + (self.sum_sub(1, pos) * pos as i64)
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - self.sum(l)
    }
}
