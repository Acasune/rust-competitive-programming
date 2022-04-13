use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Binary,
    io::*,
    str::FromStr,
};
use std::{f64, i64, usize};
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
    let vec: Vec<char> = sc.get::<String>().chars().collect();
    sc.new_line();
    let mut K: i64 = sc.get();

    let mut vec = vec
        .into_iter()
        .map(|c| c as u8 - 'a' as u8)
        .collect::<Vec<u8>>();

    let n = vec.len();
    let mut tree = FenwickTree::<i64>::new(n, 0);
    let mut idxes = vec![VecDeque::<usize>::new(); 26];
    let mut ans = vec![];
    for (i, &e) in vec.iter().enumerate() {
        idxes[e as usize].push_back(i);
    }
    for i in 0..n {
        tree.add(i, 1);
    }
    for i in 0..n {
        for j in 0..26 {
            if idxes[j].len() > 0 {
                let top = *idxes[j].front().unwrap();
                let req = tree.sum(top) - tree.sum(0);

                if req <= K {
                    ans.push((j as u8 + 'a' as u8) as char);
                    idxes[j].pop_front();
                    K -= req;
                    tree.add(top, -1);
                    break;
                }
            }
        }
    }

    println!("{}", ans.into_iter().collect::<String>());
}

struct FenwickTree<T> {
    n: usize,
    data: Vec<T>,
    e: T,
}

impl<T> FenwickTree<T>
where
    T: Clone,
    T: std::ops::AddAssign<T>,
{
    fn new(n: usize, e: T) -> Self {
        let size = n.next_power_of_two();
        FenwickTree {
            n: size,
            data: vec![e.clone(); size],
            e: e,
        }
    }
    fn add(&mut self, mut pos: usize, x: T) {
        pos += 1;
        while pos <= self.n {
            self.data[pos - 1] += x.clone();
            pos += pos & pos.wrapping_neg();
        }
    }
    fn sum(&self, mut pos: usize) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        let data = &self.data;
        let mut s = self.e.clone();
        while pos > 0 {
            s += data[pos - 1].clone();
            pos -= pos & pos.wrapping_neg();
        }
        s += data[pos].clone();
        s
    }
}
