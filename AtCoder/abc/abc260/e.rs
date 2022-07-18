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
        N:usize,M:usize,
        AB:[(usize,usize);N],
    }
    let mut inv_l = vec![vec![]; 2 * 1_00000 + 10];
    let mut inv_r = vec![vec![]; 2 * 1_00000 + 10];
    for (i, &(a, b)) in AB.iter().enumerate() {
        inv_l[a].push(i);
        inv_r[b].push(i);
    }
    let mut tree = RangeAddQueryTree::new(2 * 1_00000 + 10);
    let mut l = 1;
    let mut r = 1;
    let mut cnt = 0;
    let mut visited = vec![vec![false; 2]; N];

    while r <= M {
        let p_l = l;
        while cnt < N && r <= M {
            for &rr in &inv_l[r] {
                visited[rr][0] = true;
                cnt += 1;
            }
            for &rr in &inv_r[r] {
                visited[rr][1] = true;
                if !visited[rr][0] {
                    cnt += 1;
                }
            }
            if cnt == N {
                // println!("upper {} {}", l, r);
                tree.add(r - l + 1, M - l + 2, 1);
            }
            r += 1;
        }
        while l < r && cnt == N {
            if cnt == N && (l != p_l) {
                // println!("undr {} {} {}", p_l, l, r);
                tree.add(r - l, M - l + 2, 1);
            }
            // println!("{}", cnt);
            for &ll in &inv_l[l] {
                visited[ll][0] = false;
                if !visited[ll][1] {
                    cnt -= 1;
                }
            }
            for &ll in &inv_r[l] {
                visited[ll][1] = false;
                if !visited[ll][0] {
                    cnt -= 1;
                }
            }
            l += 1;
        }
    }
    for i in 1..=M {
        print!("{} ", tree.query(i, i + 1));
    }
    println!();
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
