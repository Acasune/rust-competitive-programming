use std::{
    cmp::{max, min},
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
    let mut n = sc.get::<i64>();
    //n *= 10;
    let mx = n;
    let m = sc.get::<usize>();
    let mut t = sc.get::<i64>();
    //t *= 10;

    sc.new_line();
    let mut pa = sc.get::<i64>();
    let mut pb = sc.get::<i64>();
    n -= pa;
    if n <= 0 {
        println!("No");
        return;
    }
    n = min(mx, n + pb - pa);

    for _ in 0..m - 1 {
        sc.new_line();
        let mut a = sc.get::<i64>();
        let mut b = sc.get::<i64>();
        n -= a - pb;
        if n <= 0 {
            println!("No");
            return;
        }
        n = min(mx, n + b - a);
        pb = b;
    }

    if t - pb < n {
        println!("Yes");
    } else {
        println!("No");
    }
}
