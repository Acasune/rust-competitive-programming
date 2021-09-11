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

    let mut v: Vec<i64> = Vec::new();

    for _ in 0..n {
        sc.new_line();
        let s: i64 = sc.get();
        v.push(s);
    }
    let mut g = {
        let mut gg = Vec::new();
        let mut x = 1;
        while x <= n {
            gg.push(x);
            x = 3 * x + 1;
        }
        gg.reverse();
        gg
    };
    shell_sort(&mut v, n as i64, g);
}

fn insertion_sort(v: &mut Vec<i64>, n: i64, g: i64, cnt: &mut i64) {
    for i in g as usize..n as usize {
        let mut j: i64 = i as i64 - g;
        let mut a = v[i];
        while j >= 0 && v[j as usize] > a {
            v[(j + g) as usize] = v[j as usize];
            j -= g;
            *cnt += 1;
        }
        v[(j + g) as usize] = a
    }
}

fn shell_sort(v: &mut Vec<i64>, n: i64, g: Vec<usize>) {
    let mut cnt = 0 as i64;
    for gg in g.iter() {
        insertion_sort(v, n, *gg as i64, &mut cnt);
    }

    println!("{}", &g.len());
    print_vec(&g);
    println!("{}", cnt);
    for vv in v {
        println!("{}", vv);
    }
}

fn print_vec(v: &Vec<usize>) {
    for i in 0..v.len() - 1 {
        print!("{} ", v[i]);
    }
    println!("{}", v[v.len() - 1]);
}
