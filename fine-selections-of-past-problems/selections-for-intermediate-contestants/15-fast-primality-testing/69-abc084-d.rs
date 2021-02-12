use std::{
    cmp::{max, min},
    collections::HashSet,
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

    let mut sieve = vec![true; 100_000 + 10];
    for i in 2..100_000 + 10 {
        if !sieve[i] {
            continue;
        }
        let mut j = i + i;
        while j <= 100_000 {
            sieve[j] = false;
            j += i;
        }
    }
    sieve[0] = false;
    sieve[1] = false;

    let mut cs = vec![0; 100_010];
    for i in 2..100_000 {
        if i % 2 == 0 {
            cs[i] = cs[i - 1];
            continue;
        }
        if sieve[i] && sieve[(i + 1) / 2] {
            cs[i] += cs[i - 1] + 1
        } else {
            cs[i] = cs[i - 1];
        }
    }

    sc.new_line();
    let q = sc.get::<i64>();

    for _ in 0..q {
        sc.new_line();
        let l = sc.get::<usize>();
        let r = sc.get::<usize>();
        println!("{}", cs[r] - cs[l - 1]);
    }
}
