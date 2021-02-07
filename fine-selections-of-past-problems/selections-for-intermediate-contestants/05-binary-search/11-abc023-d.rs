// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b

use std::{
    cmp::{max, min},
    io::*,
    str::FromStr,
    usize,
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
    let n = sc.get::<usize>();

    let mut v = vec![];

    for i in 0..n {
        sc.new_line();
        let tmp = sc.get_as_vec::<i64>();
        v.push(tmp);
    }
    let mut l = 0;
    let mut r = {
        let mut tmp = -1;
        for i in 0..n {
            tmp = max(tmp, v[i][0] + (n as i64 - 1) * v[i][1]);
        }
        tmp
    };

    while r - l > 1 {
        let m = (r + l) / 2;
        let mut tmp = vec![0; n];
        for i in 0..n {
            tmp[i] = (m - v[i][0]) / v[i][1];
            if m - v[i][0] < 0 {
                tmp[i] = -1;
            }
        }
        tmp.sort();

        let mut ok = true;
        for i in 0..n {
            if (i as i64) > tmp[i] {
                ok = false;
                break;
            }
        }
        if ok {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);
}
