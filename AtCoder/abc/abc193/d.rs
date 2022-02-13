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
    let K = sc.get::<usize>();
    sc.new_line();
    let S = sc.get::<String>().chars().collect::<Vec<char>>();
    sc.new_line();
    let T = sc.get::<String>().chars().collect::<Vec<char>>();
    let mut remain_cards = vec![K; 11];
    let mut taka_cards = vec![1; 11];
    let mut aoki_cards = vec![1; 11];
    let mut cnt = 0;
    remain_cards[0] = 0;
    taka_cards[0] = 0;
    aoki_cards[0] = 0;
    for i in 0..4 {
        let a = S[i].to_digit(10).unwrap() as usize;
        let b = T[i].to_digit(10).unwrap() as usize;
        // println!("{} {}", a, b);
        taka_cards[a] *= 10;
        aoki_cards[b] *= 10;
        remain_cards[a] -= 1;
        remain_cards[b] -= 1;
    }
    for i in 1..=9 {
        if remain_cards[i] <= 0 {
            continue;
        }
        remain_cards[i] -= 1;
        taka_cards[i] *= 10;
        for j in 1..=9 {
            if remain_cards[j] <= 0 {
                continue;
            }
            aoki_cards[j] *= 10;
            let t_points = calc(&taka_cards);
            let a_points = calc(&aoki_cards);
            if t_points > a_points {
                cnt += (remain_cards[i] + 1) * remain_cards[j];
            }
            aoki_cards[j] /= 10;
        }
        remain_cards[i] += 1;
        taka_cards[i] /= 10;
    }
    println!(
        "{}",
        cnt as f64 / ((9. * K as f64 - 8.) * (9. * K as f64 - 9.))
    );
}

fn calc(hands: &Vec<usize>) -> usize {
    let mut total = 0;
    for i in 1..=9 {
        total += hands[i] * i;
    }
    return total;
}
