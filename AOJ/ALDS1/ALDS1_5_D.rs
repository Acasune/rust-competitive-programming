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

fn merge(a: &mut Vec<i64>, left: usize, mid: usize, right: usize, cnt: &mut usize) {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut l_vec = Vec::<i64>::new();
    let mut r_vec = Vec::<i64>::new();
    for i in 0..n1 {
        l_vec.push(a[left + i]);
    }
    l_vec.push(1_000_000_000_000);
    for i in 0..n2 {
        r_vec.push(a[mid + i]);
    }
    r_vec.push(1_000_000_000_000);
    let mut n1_idx = 0;
    let mut n2_idx = 0;
    for k in left..right {
        if l_vec[n1_idx] <= r_vec[n2_idx] {
            a[k] = l_vec[n1_idx];
            n1_idx += 1;
        } else {
            *cnt += n1 - n1_idx;
            a[k] = r_vec[n2_idx];
            n2_idx += 1;
        }
    }
}

fn mergeSort(a: &mut Vec<i64>, left: usize, right: usize, cnt: &mut usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        mergeSort(a, left, mid, cnt);
        mergeSort(a, mid, right, cnt);
        merge(a, left, mid, right, cnt);
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    sc.new_line();
    let mut a: Vec<i64> = sc.get_as_vec();
    let mut cnt = 0;

    mergeSort(&mut a, 0, n, &mut cnt);
    println!("{}", cnt);
}
