use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let K: usize = sc.get();
    sc.new_line();
    let al = sc.get_as_vec::<usize>();
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut cnt = 0;
    let mut ret = 0;
    let mut mp = HashMap::new();
    while r < N {
        if !mp.contains_key(&al[r]) {
            while cnt >= K {
                *mp.get_mut(&al[l]).unwrap() -= 1;
                if *mp.get(&al[l]).unwrap() == 0 {
                    cnt -= 1;
                    mp.remove(&al[l]);
                }
                l += 1;
            }
            cnt += 1;
        }
        ret = ret.max(r - l + 1);
        *mp.entry(&al[r]).or_insert(0) += 1;
        r += 1;
    }
    println!("{}", ret);
}
