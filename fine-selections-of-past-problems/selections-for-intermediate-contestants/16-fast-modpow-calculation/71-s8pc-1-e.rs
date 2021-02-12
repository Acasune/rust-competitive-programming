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
    let q = sc.get::<u64>();
    let md = 1_000_000_007;

    sc.new_line();
    let mut a = sc.get_as_vec::<u64>();

    // from-indexed
    let mut dists = vec![0; n - 1];

    for i in 0..n - 1 {
        dists[i] = mod_pow(a[i], a[i + 1], md);
    }

    let mut cs = vec![0; n];
    for i in 1..n {
        cs[i] += (cs[i - 1] + dists[i - 1]) % md;
        cs[i] %= md;
    }

    let mut ans = 0;

    let mut c_pre = 0;
    sc.new_line();
    for i in 0..q {
        let mut c = sc.get::<usize>();
        c -= 1;
        let l = min(c_pre, c);
        let r = max(c_pre, c);
        ans += (cs[r] - cs[l] + md) % md;
        ans %= md;
        c_pre = c;
    }
    ans += (cs[c_pre] - cs[0] + md) % md;
    ans %= md;

    println!("{}", ans);
}

//return a^n%mod
fn mod_pow(mut m: u64, mut n: u64, modulo: u64) -> u64 {
    let mut ret: u64 = 1;
    while n > 0 {
        if n % 2 == 1 {
            ret = ret * m % modulo;
        }
        m = m * m % modulo;
        n = n / 2;
    }

    return ret % modulo;
}
