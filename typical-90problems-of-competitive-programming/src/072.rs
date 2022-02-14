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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let H = sc.get::<usize>();
    let W = sc.get::<usize>();
    let mut maze = vec![Vec::<char>::new(); H];

    for h in 0..H {
        sc.new_line();
        maze[h] = sc.get::<String>().chars().collect::<Vec<char>>();
    }

    let mut grobal_ans = -1;
    for sh in 0..H {
        for sw in 0..W {
            if maze[sh][sw] == '#' {
                continue;
            }
            let mut local_ans: i64 = -1;
            let mut visited = vec![vec![0; W]; H];
            rec(
                sh,
                sw,
                &mut 0,
                &mut local_ans,
                &mut visited,
                sh,
                sw,
                H,
                W,
                &maze,
            );
            grobal_ans = grobal_ans.max(local_ans);
        }
    }
    println!("{}", grobal_ans);
}

fn rec(
    h: usize,
    w: usize,
    cnt: &mut i64,
    local_ans: &mut i64,
    visited: &mut Vec<Vec<i64>>,
    gh: usize,
    gw: usize,
    H: usize,
    W: usize,
    maze: &Vec<Vec<char>>,
) {
    if *cnt != 0 && *cnt != 2 && h == gh && w == gw {
        *local_ans = *local_ans.max(cnt);
        return;
    }
    for (dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if h as i64 + dy < 0
            || H as i64 <= h as i64 + dy
            || w as i64 + dx < 0
            || W as i64 <= w as i64 + dx
        {
            continue;
        }
        let nh = (h as i64 + *dy) as usize;
        let nw = (w as i64 + *dx) as usize;
        if visited[nh][nw] == 0 && maze[nh][nw] != '#' {
            let old_val = visited[nh][nw];
            visited[nh][nw] = *cnt + 1;
            rec(
                nh,
                nw,
                &mut (*cnt + 1),
                local_ans,
                visited,
                gh,
                gw,
                H,
                W,
                maze,
            );
            visited[nh][nw] = old_val;
        }
    }
}
