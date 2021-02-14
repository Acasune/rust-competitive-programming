use std::{
    cmp::{max, min, Ordering},
    collections::HashSet,
    f64::MAX,
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
    let mut n = sc.get::<usize>();
    let mut m = sc.get::<usize>();

    let mut formers = vec![];
    let mut latters = vec![];

    for i in 0..n {
        sc.new_line();
        let mut a = sc.get_as_vec::<f64>();
        formers.push((a[0], a[1], a[2]));
    }
    for i in 0..m {
        sc.new_line();
        let mut a = sc.get_as_vec::<f64>();
        latters.push((a[0], a[1]));
    }

    let mut radiuses = vec![1000000.0; m];

    for j in 0..m {
        let x_2 = latters[j].0;
        let y_2 = latters[j].1;

        for i in 0..n {
            let x_1 = formers[i].0;
            let y_1 = formers[i].1;
            let r_1 = formers[i].2;
            let d = f64::sqrt((x_2 - x_1) * (x_2 - x_1) + (y_2 - y_1) * (y_2 - y_1));
            radiuses[j] = fmin(radiuses[j], fmax(0.0, d - r_1));
        }
    }
    let mut ans = 100000000.0;
    for &r in &formers {
        ans = fmin(ans, r.2);
    }
    for &r in &radiuses {
        ans = fmin(ans, r);
    }
    for i in 0..m {
        for j in i + 1..m {
            let x_1 = latters[i].0;
            let y_1 = latters[i].1;
            let r_1 = radiuses[i];

            let x_2 = latters[j].0;
            let y_2 = latters[j].1;
            let r_2 = radiuses[j];
            let d = f64::sqrt((x_2 - x_1) * (x_2 - x_1) + (y_2 - y_1) * (y_2 - y_1));
            if r_1 + r_2 > d {
                let reminder = r_1 + r_2 - d;
                if r_1 > r_2 {
                    if r_1 - reminder >= r_2 {
                        ans = fmin(ans, r_2);
                        radiuses[i] = r_1 - reminder;
                    } else {
                        ans = fmin(ans, d / 2.0);
                        radiuses[i] = d / 2.0;
                        radiuses[j] = d / 2.0;
                    }
                } else {
                    if r_2 - reminder > r_1 {
                        ans = fmin(ans, r_1);
                        radiuses[j] = r_2 - reminder;
                    } else {
                        ans = fmin(ans, d / 2.0);
                        radiuses[i] = d / 2.0;
                        radiuses[j] = d / 2.0;
                    }
                }
            } else {
                ans = fmin(ans, fmin(r_1, r_2));
            }
        }
    }
    println!("{:.09}", ans);
}

fn fmin(x: f64, y: f64) -> f64 {
    if x > y {
        y
    } else {
        x
    }
}

fn fmax(x: f64, y: f64) -> f64 {
    if x < y {
        y
    } else {
        x
    }
}
