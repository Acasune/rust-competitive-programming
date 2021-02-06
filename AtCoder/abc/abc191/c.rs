use std::iter::FromIterator;
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
    let mut h = sc.get::<usize>();
    let mut w = sc.get::<usize>();

    let mut field = vec![Vec::<char>::new(); h];

    for i in 0..h {
        sc.new_line();
        let tmp: Vec<char> = sc.get::<String>().chars().collect();
        field[i as usize] = tmp;
    }

    let mut counts = vec![vec![0; w]; h];

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let mut cnt = 0;
            if field[i][j] == '#' {
                cnt += 1;
            }
            if field[i + 1][j] == '#' {
                cnt += 1;
            }
            if field[i][j + 1] == '#' {
                cnt += 1;
            }
            if field[i + 1][j + 1] == '#' {
                cnt += 1;
            }
            if cnt == 4 {
                continue;
            }

            counts[i][j] = (cnt) % 2;
        }
    }

    let mut ans = 0;

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            ans += counts[i][j];
        }
    }

    println!("{}", ans);
}
