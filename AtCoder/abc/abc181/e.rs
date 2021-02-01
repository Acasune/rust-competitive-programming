use std::{cmp::{max, min}, convert::TryInto, io::*, str::{Chars, FromStr}};

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
    let mut n:usize= sc.get();
    let mut m:usize= sc.get();


    sc.new_line();
    let mut h = sc.get_as_vec::<i128>();
    sc.new_line();
    let mut w = sc.get_as_vec::<i128>();

    h.sort();

    let mut l=vec![0;n/2+1];
    let mut r=vec![0;n/2+1];

    for i in 1..n/2+1 {
        l[i] =l[i-1] + h[2*i-1] - h[2*i-2];
    }

    for i in (0..(n/2)).rev() {
        r[i] = r[i+1] + h[2*i+2] - h[2*i+1];
    }

    let mut ans:i128 = 1_000_000_010;

    for e in w {
        let idx = h.binary_search_by_key(&(e*2),|&a| a * 2 +1).err().unwrap();
        if idx==0 {
            ans = min(ans, r[0]+h[0]-e);
        } else if idx==n {
            ans = min( ans, l[n/2]+e-h[n-1]);
        } else if e==h[idx-1] {
            if idx%2==0 {
                ans = min(ans,l[idx/2]+r[idx/2]);
            } else {
                ans = min(ans,l[idx/2+1]+r[(idx-1)/2+1]);
            }
        } else {
            if (idx)%2==0 {
              ans = min(ans,l[idx/2] + (h[idx]-e) + r[idx/2]);
            } else {
              ans = min(ans,l[idx/2] + (e-h[idx-1]) + r[idx/2]);
            }
        }
    }
    println!("{}", ans);
    

}
