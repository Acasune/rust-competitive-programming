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

fn koch(p1: &Vec<f64>, p2: &Vec<f64>, stop: &usize, cnt: usize) {
    if stop == &cnt {
        return;
    }
    let sx = 2.0 * p1[0] / 3. + p2[0] / 3.;
    let sy = 2.0 * p1[1] / 3. + p2[1] / 3.;
    let tx = p1[0] / 3. + 2.0 * p2[0] / 3.;
    let ty = p1[1] / 3. + 2.0 * p2[1] / 3.;
    let radian = 60_f64.to_radians();
    let ux = (tx - sx) * radian.cos() - (ty - sy) * radian.sin() + sx;
    let uy = (tx - sx) * radian.sin() + (ty - sy) * radian.cos() + sy;
    koch(p1, &vec![sx, sy], stop, cnt + 1);
    println!("{} {}", sx, sy);
    koch(&vec![sx, sy], &vec![ux, uy], stop, cnt + 1);
    println!("{} {}", ux, uy);
    koch(&vec![ux, uy], &vec![tx, ty], stop, cnt + 1);
    println!("{} {}", tx, ty);
    koch(&vec![tx, ty], p2, stop, cnt + 1);
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();

    println!("{:.8} {:.8}", 0.000, 0.000);
    koch(&vec![0.0000, 0.000], &vec![100.0000, 0.0000], &n, 0);
    println!("{:.8} {:.8}", 100.000, 0.000);
}
