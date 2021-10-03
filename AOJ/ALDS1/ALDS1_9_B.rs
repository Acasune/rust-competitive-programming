#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::HashMap,
    collections::VecDeque,
    fmt::Display,
    rc::{Rc, Weak},
};
use std::{io::*, str::FromStr};

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

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    sc.new_line();
    let mut vv = sc.get_as_vec::<i64>();
    buildMaxHeap(&mut vv);
    for i in 0..n {
        print!(" {}", vv[i]);
    }
    println!();
}

fn maxHeapify(vv: &mut Vec<i64>, i: usize) {
    let n = vv.len();
    let l = 2 * i + 1;
    let r = 2 * i + 2;
    let mut largest = i;
    if l < n && vv[l] > vv[i] {
        largest = l;
    }
    if r < n && vv[r] > vv[largest] {
        largest = r;
    }
    if largest != i {
        vv.swap(largest, i);
        maxHeapify(vv, largest);
    }
}

fn buildMaxHeap(vv: &mut Vec<i64>) {
    for i in (0..vv.len() / 2).rev() {
        maxHeapify(vv, i);
    }
}
