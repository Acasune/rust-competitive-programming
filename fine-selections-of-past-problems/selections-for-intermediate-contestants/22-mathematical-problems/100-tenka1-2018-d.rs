use std::{
    cmp::{max, min},
    collections::HashSet,
    io::*,
    str::FromStr,
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

    let mut memo = vec![0; n + 1];
    let mut k = 1;
    while k * (k + 1) / 2 <= n {
        memo[k * (k + 1) / 2] = 1;
        k += 1;
    }
    if memo[n as usize] != 1 {
        println!("No");
        return;
    }

    let set_size = k;

    let mut subsets = vec![Vec::<usize>::new(); set_size];

    let mut n_full_subset = 0;
    let mut cs = 0;

    for i in 1..=n {
        let fst = (i - 1 + cs) / (k - 1);
        let snd = (i - 1 + cs) % (k - 1) + 1;
        subsets[fst].push(i);
        subsets[snd].push(i);
        if subsets[n_full_subset].len() == set_size - 1 {
            n_full_subset += 1;
            cs += n_full_subset;
        }
    }
    println!("Yes");
    println!("{}", set_size);

    for s in subsets {
        print!("{}", k - 1);
        for i in 0..s.len() {
            print!(" {}", s[i]);
        }
        println!("");
    }
}
