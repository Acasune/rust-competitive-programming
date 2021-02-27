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

const dxy: [(i64, i64); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    loop {
        sc.new_line();
        let wh = sc.get_as_vec::<usize>();
        if wh[0] == 0 && wh[1] == 0 {
            break;
        }

        let mut fields = Vec::new();
        let mut visited = vec![vec![false; wh[0]]; wh[1]];
        for h in 0..wh[1] {
            sc.new_line();
            let h = sc.get_as_vec();
            fields.push(h);
        }

        let mut cnt = 0;

        for v in 0..wh[0] {
            for h in 0..wh[1] {
                if visited[h][v] || fields[h][v] == 0 {
                    continue;
                }
                dfs(
                    (h as i64, v as i64),
                    &mut visited,
                    &fields,
                    wh[1] as i64,
                    wh[0] as i64,
                );
                cnt += 1;
            }
        }
        println!("{}", cnt)
    }
}

fn dfs(
    now: (i64, i64),
    visited: &mut Vec<Vec<bool>>,
    fields: &Vec<Vec<usize>>,
    h_lim: i64,
    v_lim: i64,
) {
    for (dy, dx) in dxy.iter() {
        let ny = now.0 + dy;
        let nx = now.1 + dx;
        if nx < 0 || nx >= v_lim || ny < 0 || ny >= h_lim {
            continue;
        }
        if fields[ny as usize][nx as usize] == 0 || visited[ny as usize][nx as usize] {
            continue;
        }
        visited[ny as usize][nx as usize] = true;
        dfs((ny, nx), visited, fields, h_lim, v_lim)
    }
}
