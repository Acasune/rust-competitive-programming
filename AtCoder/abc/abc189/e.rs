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

#[allow(unused_macros)]
macro_rules! scan {
  ($e:expr; $t:ty) => {
    $e.get::<$t>()
  };
  ($e:expr; $($t:ty), *) => {
    ($($e.get::<$t>(),)*)
  }
}

struct Scanner<R: BufRead> {
    reader: R,
    iter: std::vec::IntoIter<String>,
}

#[allow(dead_code)]
impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N = sc.get::<usize>();
    let mut XY = Vec::<Vec<i64>>::new();
    for _ in 0..N {
        sc.new_line();
        XY.push(sc.get_as_vec::<i64>());
    }

    let mut mat = [[0; 3]; 3];
    for (i, mat) in mat.iter_mut().enumerate() {
        mat[i] = 1;
    }
    type Mat = [[i64; 3]; 3];
    let mul = |a: &Mat, b: &Mat| -> Mat {
        let mut c = Mat::default();
        for (c, a) in c.iter_mut().zip(a.iter()) {
            for (a, b) in a.iter().zip(b.iter()) {
                for (c, b) in c.iter_mut().zip(b.iter()) {
                    *c += *a * *b;
                }
            }
        }
        c
    };
    let mut cum = vec![];
    cum.push(mat);
    sc.new_line();
    let M = sc.get::<usize>();
    let mut OP = Vec::<Vec<i64>>::new();
    for _ in 0..M {
        sc.new_line();
        OP.push(sc.get_as_vec::<i64>());
    }
    for op in OP {
        let trans = if op[0] == 1 {
            [[0, 1, 0], [-1, 0, 0], [0, 0, 1]]
        } else if op[0] == 2 {
            [[0, -1, 0], [1, 0, 0], [0, 0, 1]]
        } else if op[0] == 3 {
            [[-1, 0, 2 * op[1]], [0, 1, 0], [0, 0, 1]]
        } else {
            [[1, 0, 0], [0, -1, 2 * op[1]], [0, 0, 1]]
        };
        mat = mul(&trans, &mat);
        cum.push(mat);
    }
    sc.new_line();
    let Q = sc.get::<usize>();
    for q in 0..Q {
        sc.new_line();
        let ab = sc.get_as_vec::<usize>();
        let (a, b) = (ab[0], ab[1]);
        let (x, y) = (XY[b as usize - 1][0], XY[b as usize - 1][1]);
        let mat = cum[a as usize];
        let (x, y) = (
            mat[0][0] * x + mat[0][1] * y + mat[0][2],
            mat[1][0] * x + mat[1][1] * y + mat[1][2],
        );
        println!("{} {}", x, y);
    }
}
