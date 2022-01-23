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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let H: usize = sc.get();
    let W: usize = sc.get();
    let mut arr = Vec::<Vec<i64>>::new();
    for h in 0..H {
        sc.new_line();
        let a = sc.get_as_vec::<i64>();
        arr.push(a);
    }
    let mut columns_sum = vec![0; W];
    let mut rows_sum = vec![0; H];
    for h in 0..H {
        let mut sum = 0;
        for w in 0..W {
            sum += arr[h][w];
        }
        rows_sum[h] = sum;
    }
    for w in 0..W {
        let mut sum = 0;
        for h in 0..H {
            sum += arr[h][w];
        }
        columns_sum[w] = sum;
    }
    for h in 0..H {
        for w in 0..W {
            if w == W - 1 {
                println!("{}", columns_sum[w] + rows_sum[h] - arr[h][w]);
            } else {
                print!("{} ", columns_sum[w] + rows_sum[h] - arr[h][w]);
            }
        }
    }
}
