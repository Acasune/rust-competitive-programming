use std::{
    cmp::{max, min},
    collections::HashSet,
    io::*,
    str::FromStr,
};
use std::{collections::hash_set, iter::FromIterator};

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
    let a = sc.get::<usize>();
    let b = sc.get::<usize>();
    let c = sc.get::<usize>();
    let d = sc.get::<usize>();
    let mut primes = vec![true; 210];
    primes[0] = false;
    primes[1] = false;
    for i in 2..200 {
        if primes[i] {
            let mut k = i + i;
            while k < 201 {
                primes[k] = false;
                k += i;
            }
        }
    }
    let mut win_takahasi = false;
    for i in a..=b {
        let mut win_aoki = false;
        for j in c..=d {
            win_aoki |= primes[i + j];
        }
        if !win_aoki {
            win_takahasi = true;
            break;
        }
    }
    if win_takahasi {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
    // println!("{:?}", primes);

    // for i in 2..=200 {
    //     if primes[i] {
    //         println!("{}", i);
    //     }
    // }

    // println!("{}", ans);
}
