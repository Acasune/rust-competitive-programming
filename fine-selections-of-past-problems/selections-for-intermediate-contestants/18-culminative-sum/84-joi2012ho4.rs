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

    let edge = n + 10;

    let mut field = vec![vec![0; edge]; edge];

    for i in 0..m {
        sc.new_line();
        let a = sc.get::<usize>() - 1;
        let b = sc.get::<usize>() - 1;
        let x = sc.get::<usize>();
        field[a][b] += 1;
        field[a][b + 1] -= 1;
        field[a + x + 1][b] -= 1;
        field[a + x + 2][b + 1] += 1;
        field[a + x + 1][b + x + 2] += 1;
        field[a + x + 2][b + x + 2] -= 1;
    }

    for i in 0..n + 3 {
        for j in 1..n + 3 {
            field[i][j] += field[i][j - 1];
        }
    }
    for j in 0..n + 3 {
        for i in 1..n + 3 {
            field[i][j] += field[i - 1][j];
        }
    }
    for i in 1..n + 3 {
        for j in 1..i + 3 {
            field[i][j] += field[i - 1][j - 1];
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if field[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
