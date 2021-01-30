use std::{cmp::min, io::*, str::{Chars, FromStr}, usize};

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
    let n:usize=sc.get();

    sc.new_line();
    let mut v:Vec<i64> =sc.get_as_vec();
    v.sort();

    let mut ans=0;

    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if v[i]==v[j] || v[j]==v[k] || v[k]==v[i]{
                    continue;
                }
                if v[k] >= v[i] + v[j]||v[i] >= v[j] + v[k]||v[j] >= v[k] + v[i]{
                    continue;
                }
                ans+=1;
            }
        }

    }

    println!("{}", ans);


}
