use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::collections::VecDeque;
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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N = sc.get::<usize>();
    let M = sc.get::<usize>();
    let mut G = vec![Vec::<usize>::new(); N + 10];
    let mut H = vec![Vec::<usize>::new(); N + 10];

    for _ in 0..M {
        sc.new_line();
        let mut a = sc.get::<usize>();
        let mut b = sc.get::<usize>();
        G[a].push(b);
        H[b].push(a);
    }
    let mut path = Vec::<usize>::new();
    let mut used = vec![false; N + 1];
    for i in 1..=N {
        if !used[i] {
            dfs1(i, &mut used, &G, &mut path);
        }
    }
    path.reverse();
    let mut cnt = 0;
    let mut ans = 0;
    used = vec![false; N + 1];
    for i in path {
        if !used[i] {
            cnt = 0;
            dfs2(i, &mut cnt, &mut used, &H);
            ans += cnt * (cnt - 1) / 2;
        }
    }
    println!("{}", ans);
}

fn dfs1(i: usize, used: &mut Vec<bool>, G: &Vec<Vec<usize>>, path: &mut Vec<usize>) {
    used[i] = true;
    for &j in &G[i] {
        if !used[j] {
            dfs1(j, used, G, path);
        }
    }
    path.push(i);
}

fn dfs2(i: usize, cnt: &mut usize, used: &mut Vec<bool>, H: &Vec<Vec<usize>>) {
    used[i] = true;
    *cnt += 1;
    for &j in &H[i] {
        if !used[j] {
            dfs2(j, cnt, used, H);
        }
    }
}
