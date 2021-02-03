use std::{cmp::{max, min}, convert::TryInto, io::*, str::{Chars, FromStr}};

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

const m:i64 = 1_000_000_007;

fn main() {

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut h:usize= sc.get();
    let mut w:usize= sc.get();

   
    let mut field = Vec::<Vec<char>>::new();
    for _ in 0..h {
        sc.new_line();
        let mut tmp:String= sc.get();
        field.push(tmp.chars().collect());
    }

    let mut dp = vec![vec![0;w];h];
    let mut x_l = vec![vec![0;w];h];
    let mut y_l = vec![vec![0;w];h];
    let mut z_l = vec![vec![0;w];h];

    dp[0][0] = 1;
    x_l[0][0] = 0;
    y_l[0][0] = 0;
    z_l[0][0] = 0;

    for y in 0..h {
        for x in 0..w {
            if x>0 {
                if field[y][x]=='#' {
                    continue;
                }
                x_l[y][x] += x_l[y][x-1] + dp[y][x-1];
                x_l[y][x]%=m;
                dp[y][x]+=x_l[y][x];
                dp[y][x]%=m;
            }
            if y>0 {
                if field[y][x]=='#' {
                    continue;
                }
                y_l[y][x] += y_l[y-1][x] +  dp[y-1][x];
                y_l[y][x]%=m;
                dp[y][x]+=y_l[y][x];
                dp[y][x]%=m;
            }
            if x>0 && y>0 {
                if field[y][x]=='#' {
                    continue;
                }
                z_l[y][x] += z_l[y-1][x-1] + dp[y-1][x-1];
                z_l[y][x]%=m;
                dp[y][x]+=z_l[y][x];
                dp[y][x]%=m;
            }
        }
    }

    let ans =(dp[h-1][w-1]+m)%m;
    println!("{}", ans);



}
