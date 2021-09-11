#[allow(unused_imports)]
use std::cmp::{max, min};
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

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n: usize = sc.get();
    sc.new_line();
    let mut v: Vec<i64> = sc.get_as_vec();

    bubble_sort(&mut v);
}

fn bubble_sort(v: &mut Vec<i64>) {
    let mut cnt = 0;
    let mut is_exchange = true;
    while is_exchange {
        is_exchange = false;
        for i in 1..v.len() {
            if v[i] < v[i - 1] {
                let tmp = v[i];
                v[i] = v[i - 1];
                v[i - 1] = tmp;
                is_exchange = true;
                cnt += 1;
            }
        }
    }
    print_vec(&v);
    println!("{}", cnt);
}

fn print_vec(v: &Vec<i64>) {
    for i in 0..v.len() - 1 {
        print!("{} ", v[i]);
    }
    println!("{}", v[v.len() - 1]);
}
