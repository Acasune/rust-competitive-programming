#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::VecDeque;
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

struct ps {
    name: String,
    time: i64,
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let mut deq: VecDeque<ps> = VecDeque::new();
    let mut total = 0;

    sc.new_line();
    let n: usize = sc.get();
    let q: i64 = sc.get();
    for _ in 0..n {
        sc.new_line();
        let name = sc.get();
        let time: i64 = sc.get();
        deq.push_back(ps {
            name: name,
            time: time,
        });
    }
    while !deq.is_empty() {
        let ps = deq.pop_front().unwrap();
        if ps.time <= q {
            total += ps.time;
            println!("{} {}", ps.name, total);
        } else {
            total += q;
            deq.push_back(ps {
                name: ps.name,
                time: ps.time - q,
            })
        }
    }
}
