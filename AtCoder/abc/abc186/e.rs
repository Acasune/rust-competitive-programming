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
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - (b / a) * x, x, g)
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let Q = sc.get::<usize>();
    for i in 0..Q {
        sc.new_line();
        let N = sc.get::<i64>();
        let S = sc.get::<i64>();
        let K = sc.get::<i64>();
        let (x, y, g) = ext_gcd(K, N);
        if (N - S) % g != 0 {
            println!("{}", -1);
        } else {
            let mut ans = x * ((N - S) / g);
            while ans < 0 {
                ans += N;
            }
            println!("{}", ans % (N / g));
        }
    }
}
