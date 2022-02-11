use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i64, io::*, str::FromStr, usize};

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

fn dfs(dp: &mut Vec<Vec<usize>>, size: usize, sum: usize, idx: usize, al: &Vec<usize>, N: usize) {
    if idx >= N {
        return;
    }
    //use
    dp[size + 1].push(sum + al[idx]);
    dfs(dp, size + 1, sum + al[idx], idx + 1, al, N);
    //not use
    dfs(dp, size, sum, idx + 1, al, N);
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let K: usize = sc.get();
    let P: usize = sc.get();
    let mid = N / 2;
    sc.new_line();
    let mut al = sc.get_as_vec::<usize>();
    let mut dp1 = vec![Vec::<usize>::new(); 50];
    let mut dp2 = vec![Vec::<usize>::new(); 50];

    dfs(&mut dp1, 0, 0, 0, &al, mid);
    dfs(&mut dp2, 0, 0, mid, &al, N);
    for i in 0..dp2.len() {
        dp2[i].push(usize::MAX / 10);
        dp2[i].sort();
    }
    dp1[0].push(0);
    dp2[0].push(0);
    let mut ans = 0;
    for k in 0..=K {
        for v in &dp1[k] {
            if v > &P {
                continue;
            }
            if k == K {
                ans += 1;
            } else {
                let n_r = dp2[K - k]
                    .binary_search_by_key(&((P - v + 1) * 2), |&a| 2 * a + 1)
                    .err()
                    .unwrap();

                ans += n_r;
            }
        }
    }
    println!("{}", ans);
}
