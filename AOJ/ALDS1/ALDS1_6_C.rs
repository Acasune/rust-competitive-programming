#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::HashSet,
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

#[derive(Copy, Clone, PartialEq, Eq)]
struct Card {
    info: char,
    val: i64,
}

fn partition(a: &mut Vec<Card>, p: usize, r: usize) -> usize {
    let x = a[r].clone();
    let mut i: i64 = p as i64 - 1;
    for j in p..r {
        if a[j].val <= x.val {
            i += 1;
            a.swap(i as usize, j);
        }
    }
    a.swap(i as usize + 1, r);
    return i as usize + 1;
}

fn quick_sort(a: &mut Vec<Card>, p: usize, r: usize) {
    if p < r {
        let q = partition(a, p, r);
        quick_sort(a, p, q - 1);
        quick_sort(a, q, r);
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let mut cards = Vec::<Card>::new();
    for _ in 0..n {
        sc.new_line();
        let pic: char = sc.get();
        let val: i64 = sc.get();
        let card = Card {
            info: pic,
            val: val,
        };
        cards.push(card);
    }
    let mut s_cards = cards.clone();
    quick_sort(&mut cards, 0, n - 1);
    s_cards.sort_by_key(|cards| cards.val);

    if s_cards.eq(&cards) {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    for i in 0..n {
        println!("{} {}", cards[i].info, cards[i].val);
    }
}
