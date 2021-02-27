use std::{cmp::max, io::*, str::FromStr};

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

const dyx: [(i64, i64); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    sc.new_line();
    let m = sc.get::<usize>();

    let mut fields = Vec::new();
    let mut visited = vec![false; n];

    for i in 0..m {
        sc.new_line();
        let a = sc.get_as_vec::<usize>();
        fields.push(a);
    }

    let mut ans = 0;

    for i in 0..m as i64 {
        for j in 0..n as i64 {
            if fields[i as usize][j as usize] == 0 {
                continue;
            }
            let mut visited = vec![vec![false; n]; m];
            ans = max(ans, dfs((i, j), (m, n), &mut visited, &mut fields));
        }
    }
    println!("{}", ans);
}

fn dfs(
    now: (i64, i64),
    nm: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    fields: &mut Vec<Vec<usize>>,
) -> usize {
    let mut res = 0;
    visited[now.0 as usize][now.1 as usize] = true;
    for (dy, dx) in &dyx {
        let ny = now.0 + dy;
        let nx = now.1 + dx;
        if ny < 0 || ny >= nm.0 as i64 || nx < 0 || nx >= nm.1 as i64 {
            continue;
        }
        if visited[ny as usize][nx as usize] || fields[ny as usize][nx as usize] == 0 {
            continue;
        }
        res = max(res, dfs((ny, nx), nm, visited, fields));
    }
    visited[now.0 as usize][now.1 as usize] = false;
    res += 1;
    return res;
}
