use std::{cmp::{max, min}, collections::VecDeque, io::*, str::FromStr};

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
    let mut n:usize =sc.get();
    let mut m:usize =sc.get();
    let mut k:i64 =sc.get();

    sc.new_line();
    let mut a_l:Vec<i64> =sc.get_as_vec();
    sc.new_line();
    let mut b_l:Vec<i64> =sc.get_as_vec();

    let mut a = vec![0i64];
    for i in 0..a_l.len() {
        a.push(a[i]+a_l[i]);
    }

    let mut b = vec![0i64];
    for i in 0..b_l.len() {
        b.push(b[i]+b_l[i]);
    }

    let mut a_idx=0usize;
    let mut b_idx=0usize;
    let mut ans =0;

    while a_idx<=n &&a[a_idx] <= k  {
        let d=k-a[a_idx];
        let mut l:usize =0;
        let mut r:usize=b.len() as usize;
        while r -l> 1 {
            let m = (r+l)/2;
            if b[m] > d {
                r =m;
            } else {
                l =m;
            }
        }
        // println!("{} {} {}",a[a_idx],a_idx, r-1);
        ans = max(ans,a_idx+l);
        a_idx+=1;

    }


    println!("{}", ans);



}
