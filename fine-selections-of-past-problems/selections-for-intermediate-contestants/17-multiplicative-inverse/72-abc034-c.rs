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
    let w = sc.get::<u64>();
    let h = sc.get::<u64>();
    let md = 1_000_000_007;

    println!("{}", com(w + h - 2, h - 1, md));
}

fn com(n: u64, k: u64, md: u64) -> u64 {
    let max = 510_000;
    let mut fac = vec![0 as u64; max];
    let mut inv = vec![0 as u64; max];
    let mut finv = vec![0 as u64; max];
    fac[0] = 1;
    fac[1] = 1;
    inv[0] = 1;
    inv[1] = 1;
    finv[1] = 1;
    for i in 2..max {
        fac[i] = fac[i - 1] * (i as u64) % md;
        inv[i] = md - inv[(md % (i as u64)) as usize] * (md / (i as u64)) % md;
        finv[i] = finv[i - 1] * inv[i] % md;
    }

    return {
        if n < k {
            0
        } else if n < 0 || k < 0 {
            0
        } else {
            fac[n as usize] * (finv[k as usize] * finv[n as usize - k as usize] % md) % md
        }
    };
}
