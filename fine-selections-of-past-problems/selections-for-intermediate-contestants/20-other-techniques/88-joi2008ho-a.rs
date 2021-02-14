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
    let mut n = sc.get::<usize>();
    let mut a = vec![];

    for i in 0..n {
        sc.new_line();
        let tmp = sc.get::<usize>();
        a.push(tmp);
    }

    let mut memo = vec![0; n];
    let mut right_idx = 0;
    memo[0] = 1;
    let mut left_color = a[0];
    let mut pre_color = a[0];

    for i in 1..n {
        if pre_color == a[i] {
            memo[right_idx] += 1;
        } else if i % 2 == 1 {
            // i%2==1 indicates i is an even number because of shift of 0-indexed
            if right_idx == 0 {
                memo[right_idx] += 1;
                left_color = a[i];
            } else {
                memo[right_idx - 1] += memo[right_idx] + 1;
                right_idx -= 1;
            }
        } else {
            right_idx += 1;
            memo[right_idx] = 1;
        }
        pre_color = a[i];
    }
    let mut ans = 0;
    let mut w_idx = left_color;
    while w_idx <= right_idx {
        ans += memo[w_idx];
        w_idx += 2;
    }

    println!("{}", ans);
}
