use std::{
    cmp::{max, min, Reverse},
    collections::{BinaryHeap, VecDeque},
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
    let H: usize = sc.get();
    let W: usize = sc.get();

    sc.new_line();
    let CH = sc.get::<usize>() - 1;
    let CW = sc.get::<usize>() - 1;

    sc.new_line();
    let DH = sc.get::<usize>() - 1;
    let DW = sc.get::<usize>() - 1;

    let mut maze = vec![Vec::<char>::new(); H];
    let mut visited = vec![vec![100_000_000_000; W]; H];

    for h in 0..H {
        sc.new_line();
        maze[h] = sc.get::<String>().chars().collect::<Vec<char>>();
    }
    let mut que = BinaryHeap::<Reverse<(usize, usize, usize)>>::new();
    que.push(Reverse((0, CH, CW)));

    while !que.is_empty() {
        let Reverse((cnt, h, w)) = que.pop().unwrap();
        if maze[h][w] == '#' || visited[h][w] <= cnt {
            continue;
        }
        visited[h][w] = cnt;
        for j in -2..=2 {
            for i in -2..=2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let nh = h as i64 + j;
                let nw = w as i64 + i;
                if nh < 0 || H as i64 <= nh || nw < 0 || W as i64 <= nw {
                    continue;
                }
                if maze[nh as usize][nw as usize] == '#' {
                    continue;
                }
                if (i64::abs(i) == 1 && j == 0) || (i64::abs(j) == 1 && i == 0) {
                    if visited[nh as usize][nw as usize] <= cnt {
                        continue;
                    }
                    que.push(Reverse((cnt, nh as usize, nw as usize)));
                } else if cnt + 1 < visited[nh as usize][nw as usize] {
                    que.push(Reverse((cnt + 1, nh as usize, nw as usize)));
                }
            }
        }
    }
    println!(
        "{}",
        if visited[DH][DW] != 100_000_000_000 {
            visited[DH][DW] as i64
        } else {
            -1
        }
    );
}
