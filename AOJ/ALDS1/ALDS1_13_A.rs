#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::BinaryHeap,
    collections::HashMap,
    collections::VecDeque,
    fmt::Display,
    rc::{Rc, Weak},
};
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
    pub fn new(reader: R) -> Scanner<R> {
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

struct State {
    nq: usize,
    field: Vec<Vec<char>>,
    hq: Vec<bool>,
    vq: Vec<bool>,
}

impl State {
    fn new(n: usize) -> Self {
        State {
            nq: n,
            field: vec![vec!['.'; 8]; 8],
            hq: vec![false; 8],
            vq: vec![false; 8],
        }
    }
}

fn dfs(ans: &mut Vec<Vec<char>>, state: &mut State) -> bool {
    if state.nq == 8 {
        if !is_valid(state) {
            return false;
        }
        for j in 0..8 {
            for i in 0..8 {
                ans[j][i] = state.field[j][i];
            }
        }
        return true;
    }
    for j in 0..8 {
        for i in 0..8 {
            if state.hq[j] || state.vq[i] {
                continue;
            }
            state.hq[j] = true;
            state.vq[i] = true;
            state.field[j][i] = 'Q';
            state.nq += 1;
            let flg = dfs(ans, state);
            if flg {
                return true;
            }
            state.nq -= 1;
            state.hq[j] = false;
            state.vq[i] = false;
            state.field[j][i] = '.';
        }
    }
    return false;
}

fn is_valid(state: &mut State) -> bool {
    for a in 0..8 {
        if state.field[a].iter().filter(|&&c| c == 'Q').count() > 1 {
            return false;
        }
    }
    for b in 0..8 {
        if (0..8).filter(|&i| state.field[b][i] == 'Q').count() > 1 {
            return false;
        }
    }
    let mut cnt = 0;
    // - 45 deg
    for v in (0..7).rev() {
        let loop_cnt = 8 - v;
        cnt = 0;
        for i in 0..loop_cnt {
            if state.field[i][v + i] == 'Q' {
                cnt += 1;
            }
        }
        if cnt > 1 {
            return false;
        }
    }
    for h in (1..7) {
        let loop_cnt = 8 - h;
        cnt = 0;
        for i in 0..loop_cnt {
            if state.field[h + i][i] == 'Q' {
                cnt += 1;
            }
        }
        if cnt > 1 {
            return false;
        }
    }

    // 45 deg
    for h in 0..8 {
        let loop_cnt = h + 1;
        cnt = 0;
        for i in 0..loop_cnt {
            if state.field[h - i][i] == 'Q' {
                cnt += 1;
            }
        }
        if cnt > 1 {
            return false;
        }
    }
    for v in (1..7).rev() {
        let loop_cnt = 8 - v;
        cnt = 0;
        for i in 0..loop_cnt {
            if state.field[7 - i][v + i] == 'Q' {
                cnt += 1;
            }
        }
        if cnt > 1 {
            return false;
        }
    }
    return true;
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let mut state = State::new(n);
    let mut ans = vec![vec!['.'; 8]; 8];
    for _ in 0..n {
        sc.new_line();
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        state.hq[a] = true;
        state.vq[b] = true;
        state.field[a][b] = 'Q';
    }
    dfs(&mut ans, &mut state);

    for j in 0..8 {
        println!(
            "{}",
            ans[j]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}
