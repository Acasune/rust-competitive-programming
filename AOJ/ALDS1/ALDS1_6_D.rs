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
    let mut v: Vec<usize> = sc.get_as_vec();
    let mut s_v = v.clone();
    s_v.sort();
    let gmc = s_v[0];
    let mut memo = vec![false; n];
    let mut mp = HashMap::<usize, usize>::new();
    let mut ans = 0;
    for i in 0..n {
        if v[i] == s_v[i] {
            memo[i] = true;
        } else {
            mp.insert(v[i], i);
        }
    }
    for i in 0..n {
        if memo[i] {
            continue;
        }
        let mut idx = i;
        let mut lmc = 1_000_000;
        let mut ltc = 0;
        let mut cnt = 0;
        loop {
            if memo[idx] {
                break;
            }
            cnt += 1;
            lmc = min(lmc, v[idx]);
            ltc += v[idx];
            memo[idx] = true;
            idx = *mp.get(&s_v[idx]).unwrap();
        }
        ans += min(ltc + lmc * (cnt - 2), ltc + lmc + gmc * (cnt + 1));
    }
    println!("{}", ans);
}
