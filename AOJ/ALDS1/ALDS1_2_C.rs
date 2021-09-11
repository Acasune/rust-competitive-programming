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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    info: String,
    value: i64,
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n: usize = sc.get();

    let mut v: Vec<Card> = Vec::new();
    sc.new_line();
    for _ in 0..n {
        let s: String = sc.get();
        v.push(Card {
            info: s.clone(),
            value: s.chars().nth(1).unwrap() as i64 - '0' as i64,
        });
    }
    let mut w = v.clone();
    bubble_sort(&mut v);
    selection_sort(&mut w);

    print_vec(&mut v);
    println!("Stable");
    print_vec(&mut w);
    if v == w {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}

fn bubble_sort(v: &mut Vec<Card>) {
    for i in 0..v.len() {
        for j in (i + 1..v.len()).rev() {
            if v[j].value < v[j - 1].value {
                v.swap(j, j - 1)
            }
        }
    }
}

fn selection_sort(v: &mut Vec<Card>) {
    let len: usize = v.len();

    for i in 0..len {
        let mut min = v[i].value;
        let mut idx = i;
        for j in i + 1..len {
            if v[j].value < min {
                min = v[j].value;
                idx = j;
            }
        }
        if v[i].value > v[idx].value {
            v.swap(i, idx);
        }
    }
}

fn print_vec(v: &mut Vec<Card>) {
    for i in 0..v.len() - 1 {
        print!("{} ", v[i].info);
    }
    println!("{}", v[v.len() - 1].info);
}
