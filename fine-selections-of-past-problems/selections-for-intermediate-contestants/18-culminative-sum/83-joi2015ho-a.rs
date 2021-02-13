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

    sc.new_line();
    let n = sc.get::<usize>();
    let m = sc.get::<usize>();

    sc.new_line();
    let ps = sc.get_as_vec::<usize>();

    // Count how many each station is used;

    let mut stations = vec![0 as i64; n];

    let mut prev = ps[0] - 1;

    for i in 1..ps.len() {
        let next = ps[i] - 1;
        let l = min(prev, next);
        let r = max(prev, next);
        stations[l] += 1;
        stations[r] -= 1;
        prev = next;
    }
    for i in 1..n {
        stations[i] += stations[i - 1];
    }

    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n];

    for i in 0..n - 1 {
        sc.new_line();
        let v = sc.get_as_vec::<i64>();
        a[i] = v[0];
        b[i] = v[1];
        c[i] = v[2];
    }

    let mut ans = 0;
    for i in 0..n {
        ans += min(a[i] * stations[i], b[i] * stations[i] + c[i])
    }

    println!("{}", ans);
}
