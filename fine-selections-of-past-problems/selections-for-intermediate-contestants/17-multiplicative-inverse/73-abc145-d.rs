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
    let x = sc.get::<u64>();
    let y = sc.get::<u64>();

    if (x + y) % 3 != 0 || (max(x, y) + min(x, y) - 1) / min(x, y) > 2 {
        println!("0");
        return;
    }

    // num of arrows 1
    let a = (2 * x - y) / 3;
    // num of arrows 2
    let b = (2 * y - x) / 3;

    if a == 0 || b == 0 {
        println!("1");
        return;
    }

    let md = 1_000_000_007;

    println!("{}", com(a + b, min(a, b), md));
}

fn com(n: u64, k: u64, md: u64) -> u64 {
    let max = 1_000_010;
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
