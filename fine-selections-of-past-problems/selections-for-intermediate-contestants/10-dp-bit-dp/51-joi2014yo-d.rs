use std::{cmp::min, io::*, str::FromStr};

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

    let md = 10007;

    sc.new_line();
    let n = sc.get::<usize>();
    sc.new_line();
    let ps: Vec<char> = sc.get::<String>().chars().collect();
    let mut pic = vec![];
    for e in ps {
        if e == 'J' {
            pic.push(0);
        } else if e == 'O' {
            pic.push(1);
        } else {
            pic.push(2);
        }
    }

    let mut dp = vec![vec![0; n]; (1 << 3)];

    for s in 0..(1 << 3) {
        if s & (1 << 0) > 0 && s & (1 << pic[0]) > 0 {
            dp[s][0] = 1;
        }
    }

    for d in 1..n {
        for s in 0..(1 << 3) {
            if s & (1 << pic[d]) == 0 {
                continue;
            }
            for s_1 in 0..(1 << 3) {
                if s & s_1 > 0 {
                    dp[s][d] += dp[s_1][d - 1];
                    dp[s][d] %= md;
                }
            }
        }
    }

    println!("{}", {
        let mut ans = 0;
        for s in 0..(1 << 3) {
            ans += dp[s][n - 1];
            ans %= md;
        }
        ans
    });
}
