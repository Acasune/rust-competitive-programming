use std::{
    cmp::{max, min},
    io::*,
    str::{Chars, FromStr},
    vec,
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
    let c: i64 = sc.get();

    let mut sm = vec![];

    for i in 0..n {
        sc.new_line();
        let a = sc.get_as_vec::<i64>();
        sm.push(vec![a[0], a[2]]);
        sm.push(vec![a[1] + 1, -a[2]]);
    }
    sm.sort();

    let mut ans = 0;
    let mut now_price = 0;
    let mut cur = 0;
    for e in sm {
        ans += (e[0] - cur) * min(now_price, c);
        now_price += e[1];
        cur = e[0];
    }
    println!("{}", ans);
}
