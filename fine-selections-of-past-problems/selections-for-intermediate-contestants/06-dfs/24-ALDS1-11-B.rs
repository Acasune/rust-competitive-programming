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
    let n = sc.get::<usize>();

    let mut records = vec![vec![0; 2]; n];
    let mut edges = vec![Vec::new(); n];

    for i in 0..n {
        sc.new_line();
        let a = sc.get_as_vec::<usize>();
        for i in 2..a[1] + 2 {
            edges[a[0] - 1].push(a[i] - 1);
        }
    }

    let mut time = 0;

    for i in 0..n {
        if records[i][0] == 0 {
            dfs(i, &mut time, &edges, &mut records);
        }
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, records[i][0], records[i][1])
    }
}

fn dfs(now: usize, t: &mut usize, edges: &Vec<Vec<usize>>, records: &mut Vec<Vec<usize>>) {
    *t += 1;
    records[now][0] = *t;
    for i in edges[now].clone() {
        if records[i][0] == 0 {
            dfs(i.clone(), t, edges, records)
        }
    }
    *t += 1;
    records[now][1] = *t;
}
