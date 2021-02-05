use std::{io::*, str::FromStr, usize};

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
    let mut n = sc.get::<usize>();
    sc.new_line();
    let mut a = sc.get_as_vec::<i64>();

    let left = a[0..(1 << (n - 1))]
        .iter()
        .enumerate()
        .max_by_key(|&(i, &a)| a)
        .unwrap()
        .0;
    let right = a[(1 << (n - 1))..]
        .iter()
        .enumerate()
        .max_by_key(|&(i, &a)| a)
        .unwrap()
        .0;

    let ans = {
        if a[left] > a[right + (1 << (n - 1))] {
            right + 1 + (1 << (n - 1))
        } else {
            left + 1
        }
    };

    println!("{}", ans);
}
