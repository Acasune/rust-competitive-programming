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

    loop {
        sc.new_line();
        let n = sc.get::<usize>();
        if n == 0 {
            break;
        }

        // Convert time to seconds
        let range = 24 * 60 * 60;

        let mut cs = vec![0; range + 3];

        for _ in 0..n {
            sc.new_line();
            let a = sc.get::<String>();
            let a: Vec<&str> = a.split(':').collect();
            let b = sc.get::<String>();
            let b: Vec<&str> = b.split(':').collect();
            let l = a[0].parse::<usize>().unwrap() * 60 * 60
                + a[1].parse::<usize>().unwrap() * 60
                + a[2].parse::<usize>().unwrap();
            let r = b[0].parse::<usize>().unwrap() * 60 * 60
                + b[1].parse::<usize>().unwrap() * 60
                + b[2].parse::<usize>().unwrap();
            cs[l] += 1;
            cs[r] -= 1;
        }
        for i in 1..range + 1 {
            cs[i] += cs[i - 1];
        }
        let ans = cs.iter().max().unwrap();

        println!("{}", ans);
    }
}
