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

struct PriorityQueue {
    mx: usize,
    sz: usize,
    heap: Vec<i32>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            mx: 2_000_000_000,
            sz: 0,
            heap: vec![0; 2_000_000_000],
        }
    }
    fn push(&mut self, x: i32) {
        let mut i = self.sz;
        self.sz += 1;
        while i > 0 {
            let pa = (i - 1) / 2;
            if self.heap[pa] > x {
                break;
            }
            self.heap[i] = self.heap[pa];
            i = pa;
        }
        self.heap[i] = x;
    }
    fn pop(&mut self) -> i32 {
        let ret = self.heap[0];
        self.sz -= 1;
        let mut x = self.heap[self.sz];
        let mut i = 0;
        while 2 * i + 1 < self.sz {
            let mut a = 2 * i + 1;
            let mut b = 2 * i + 2;
            if b < self.sz && self.heap[b] > self.heap[a] {
                a = b;
            }
            if self.heap[a] <= x {
                break;
            }
            self.heap[i] = self.heap[a];
            i = a;
        }
        self.heap[i] = x;
        return ret;
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let mut pque = PriorityQueue::new();

    loop {
        sc.new_line();
        let op: &str = &sc.get::<String>();
        match op {
            "insert" => pque.push(sc.get::<i32>()),
            "extract" => println!("{}", pque.pop()),
            "end" => {
                break;
            }
            _ => panic!(),
        }
    }
}
