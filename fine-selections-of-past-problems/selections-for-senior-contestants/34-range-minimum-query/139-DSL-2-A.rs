#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i32, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

struct PURQ<T, F> {
    n: usize,
    data: Vec<T>,
    e: T,
    op: F,
}

impl<T, F> PURQ<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    fn new(n: usize, e: T, op: F) -> Self {
        let size = n.next_power_of_two();
        PURQ {
            n: size,
            data: vec![e.clone(); 2 * size],
            e: e,
            op: op,
        }
    }
    fn update(&mut self, pos: usize, v: T) {
        let mut pos = pos + self.n;
        let data = &mut self.data;
        data[pos] = v;
        pos >>= 1;
        while pos > 0 {
            data[pos] = (self.op)(&data[2 * pos], &data[2 * pos + 1]);
            pos >>= 1;
        }
    }
    fn update_tmp(&mut self, pos: usize, v: T) {
        self.data[pos] = v;
    }
    fn update_all(&mut self) {
        let data = &mut self.data;
        for i in (0..self.n).rev() {
            data[i] = (self.op)(&data[2 * i], &data[2 * i + 1]);
        }
    }
    fn find(&self, l: usize, r: usize) -> T {
        if l == r {
            return self.e.clone();
        }
        let mut p = self.e.clone();
        let mut q = self.e.clone();
        let mut l = l + self.n;
        let mut r = r + self.n;
        let data = &self.data;
        while l < r {
            if l & 1 == 1 {
                p = (self.op)(&p, &data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                q = (self.op)(&q, &data[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&p, &q)
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    sc.new_line();
    let n = sc.get::<usize>();
    let q = sc.get::<usize>();
    let mut tree = PURQ::new(n, i32::MAX as i64, |&a, &b| a.min(b));
    for _ in 0..q {
        sc.new_line();
        let op = sc.get::<usize>();
        let x = sc.get::<usize>();
        let y = sc.get::<usize>();
        if op == 0 {
            tree.update(x, y as i64);
        } else {
            println!("{}", tree.find(x, y + 1));
        }
    }
}

use std::{io::*, str::FromStr};

struct Scanner<R: BufRead> {
    reader: R,
    iter: std::vec::IntoIter<String>,
}

#[allow(dead_code)]
impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Scanner<R> {
        Scanner {
            reader,
            iter: vec![String::new()].into_iter(),
        }
    }
    fn new_line(&mut self) {
        let mut line = String::new();
        self.reader.read_line(&mut line).ok();
        self.iter = line
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter();
    }
    fn get<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
    fn get_as_vec<T: FromStr>(&mut self) -> Vec<T> {
        self.iter.clone().map(|v| v.parse().ok().unwrap()).collect()
    }
    fn get_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).ok();
        line.trim().to_string()
    }
}
