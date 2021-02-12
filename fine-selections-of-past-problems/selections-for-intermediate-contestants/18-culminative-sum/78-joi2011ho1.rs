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
    let h = sc.get::<usize>();
    let w = sc.get::<usize>();

    let mut j_cs = vec![vec![0; w + 1]; h + 1];
    let mut o_cs = vec![vec![0; w + 1]; h + 1];
    let mut i_cs = vec![vec![0; w + 1]; h + 1];

    sc.new_line();
    let k = sc.get::<usize>();
    let mut field = vec![];
    for m in 0..h {
        sc.new_line();
        let row: Vec<char> = sc.get::<String>().chars().collect();
        field.push(row);
    }

    for r in 0..h {
        for c in 0..w {
            if field[r][c] == 'J' {
                j_cs[r + 1][c + 1] = j_cs[r + 1][c] + 1;
                o_cs[r + 1][c + 1] = o_cs[r + 1][c];
                i_cs[r + 1][c + 1] = i_cs[r + 1][c];
            } else if field[r][c] == 'O' {
                j_cs[r + 1][c + 1] = j_cs[r + 1][c];
                o_cs[r + 1][c + 1] = o_cs[r + 1][c] + 1;
                i_cs[r + 1][c + 1] = i_cs[r + 1][c];
            } else if field[r][c] == 'I' {
                j_cs[r + 1][c + 1] = j_cs[r + 1][c];
                o_cs[r + 1][c + 1] = o_cs[r + 1][c];
                i_cs[r + 1][c + 1] = i_cs[r + 1][c] + 1;
            }
        }
    }
    for c in 0..w {
        for r in 0..h {
            j_cs[r + 1][c + 1] += j_cs[r][c + 1];
            o_cs[r + 1][c + 1] += o_cs[r][c + 1];
            i_cs[r + 1][c + 1] += i_cs[r][c + 1];
        }
    }

    for m in 0..k {
        sc.new_line();
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        let c = sc.get::<usize>();
        let d = sc.get::<usize>();
        println!(
            "{} {} {}",
            j_cs[c][d] - j_cs[a - 1][d] - j_cs[c][b - 1] + j_cs[a - 1][b - 1],
            o_cs[c][d] - o_cs[a - 1][d] - o_cs[c][b - 1] + o_cs[a - 1][b - 1],
            i_cs[c][d] - i_cs[a - 1][d] - i_cs[c][b - 1] + i_cs[a - 1][b - 1]
        )
    }
}
