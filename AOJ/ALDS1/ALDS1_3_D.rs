#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
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
    let s: Vec<char> = sc.get::<String>().chars().collect();
    let mut dq = VecDeque::new();
    let mut dq1 = VecDeque::new();
    let mut dq_s = VecDeque::new();
    for i in 0..s.len() {
        let c = s[i];
        if c == '\\' {
            dq.push_back(i);
        } else if c == '_' {
        } else if c == '/' {
            match (dq.pop_back(), dq1.pop_back()) {
                (None, _) => {}
                (Some(l1), None) => {
                    // Register first pond
                    dq1.push_back(l1);
                    dq_s.push_back(i - l1);
                }
                (Some(l1), Some(l2)) => {
                    if l1 >= l2 {
                        // New pond
                        dq1.push_back(l2);
                        dq1.push_back(l1);
                        dq_s.push_back(i - l1);
                    } else {
                        // Merge pond
                        let mut s_cum = dq_s.pop_back().unwrap() + i - l1;
                        while true {
                            match dq1.pop_back() {
                                None => {
                                    dq1.push_back(l1);
                                    dq_s.push_back(s_cum);
                                    break;
                                }
                                Some(tmp) => {
                                    if tmp <= l1 {
                                        // Cannot merge ponds more
                                        dq1.push_back(tmp);
                                        dq1.push_back(l1);
                                        dq_s.push_back(s_cum);
                                        break;
                                    } else {
                                        // Merge a pond
                                        s_cum += dq_s.pop_back().unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            panic!();
        }
    }

    if dq_s.len() == 0 {
        println!("0");
        println!("0");
    } else {
        println!("{}", dq_s.iter().sum::<usize>());
        print!("{}", dq_s.len());
        for e in dq_s {
            print!(" {}", e);
        }
        println!();
    }
}
