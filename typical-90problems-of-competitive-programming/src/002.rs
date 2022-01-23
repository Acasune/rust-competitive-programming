use std::cmp::{max, min};
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
    let N: i64 = sc.get();
    if N % 2 == 1 {
        println!("");
        return;
    }
    let mut cs = Vec::<char>::new();
    dfs(0, 0, N, &mut cs);
}

fn dfs(left: i64, right: i64, N: i64, cs: &mut Vec<char>) {
    if left * 2 == N && right * 2 == N {
        println!("{}", cs.iter().collect::<String>());
        return;
    }
    if left * 2 != N {
        cs.push('(');
        dfs(left + 1, right, N, cs);
        cs.pop();
    }
    if left > right && right * 2 != N {
        cs.push(')');
        dfs(left, right + 1, N, cs);
        cs.pop();
    }
}
