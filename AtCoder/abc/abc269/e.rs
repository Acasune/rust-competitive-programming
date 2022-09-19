#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
use std::{
    collections::HashSet,
    io::{self, *},
    str::FromStr,
};
use std::{f64, i64};

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
    let mut N: i64 = sc.get();

    // let mut x = 1_000;
    // while x > 0 {
    //     x /= 2;
    //     println!("{}", x);
    // }
    let mut n_r = N - 1;
    let mut a = 1;
    let mut b = N;
    let mut c = 1;
    let mut d = N;
    while b != a {
        let m = (a + b) / 2;
        print!("? {} {} {} {}\n", a, m, c, d);
        io::stdout().flush().unwrap();
        sc.new_line();
        let mut t: i64 = sc.get();
        if m - a + 1 > t {
            // a<few r<m
            b = m;
            n_r = t
        } else {
            a = m + 1;
            n_r = n_r - t;
        }
        // println!("hhh {} {}", a, b);
    }
    // println!("ab {} {}", a, b);
    let aa = a;
    let bb = b;
    a = 1;
    b = N;
    n_r = N - 1;
    while d != c {
        let m = (c + d) / 2;
        print!("? {} {} {} {}\n", a, b, c, m);
        io::stdout().flush().unwrap();
        sc.new_line();
        let mut t: i64 = sc.get();
        if m - c + 1 > t {
            // a<few r<m
            d = m;
            n_r = t
        } else {
            c = m + 1;
            n_r = n_r - t;
        }
        // println!("hhh {} {}", a, b);
    }
    print!("! {} {}\n", aa, c);
    io::stdout().flush().unwrap();
    return;
}
