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
    let p = sc.get::<i64>();
    let v = sc.get::<i64>();

    let mut cs = vec![vec![0; w + 1]; h + 1];

    let mut field = vec![];
    for i in 0..h {
        sc.new_line();
        let tmp = sc.get_as_vec::<i64>();
        field.push(tmp);
    }

    for i in 0..h {
        for j in 0..w {
            cs[i + 1][j + 1] = cs[i + 1][j] + field[i][j];
        }
    }
    for j in 1..=w {
        for i in 0..h {
            cs[i + 1][j] += cs[i][j];
        }
    }

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            for k in i + 1..h + 1 {
                for l in j + 1..w + 1 {
                    let field_price = cs[k][l] - cs[i][l] - cs[k][j] + cs[i][j];
                    let field_size = (k - i) * (l - j);
                    if field_price + field_size as i64 * p <= v {
                        ans = max(ans, field_size)
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
