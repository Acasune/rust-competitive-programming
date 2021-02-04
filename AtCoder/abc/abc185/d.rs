use std::{
    cmp::{max, min},
    io::*,
    str::{Chars, FromStr},
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
    let n = sc.get::<usize>();
    let m = sc.get::<usize>();

    sc.new_line();
    let mut a = sc.get_as_vec::<usize>();
    a.sort();

    if m == 0 {
        println!("1");
        return;
    }

    let mut v = Vec::<usize>::new();
    v.push(a[0] - 1);
    for i in 1..m {
        v.push(a[i] - a[i - 1] - 1);
    }
    if a.len() > 0 {
        v.push(n - a[m - 1]);
    }

    let mut vv: Vec<usize> = v.iter().filter(|&x| x > &0).cloned().collect();
    vv.sort();
    if vv.len() == 0 {
        println!("0");
        return;
    }
    let k = vv[0];
    let mut ans = 1;
    for i in 1..vv.len() {
        ans += (vv[i] + k - 1) / k;
    }
    println!("{}", ans);
}
