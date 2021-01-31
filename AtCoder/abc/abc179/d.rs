use std::{io::*, str::{Chars, FromStr}};

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

    let m = 998244353;

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut n=sc.get::<usize>();
    let mut k=sc.get::<usize>();
    let mut section=vec![];
    let mut dp=vec![0;n];
    
    for _ in 0..k {
        sc.new_line();
        let mut l:usize=sc.get();
        let mut r:usize=sc.get();
        section.push((l,r+1));
    }
    
    dp[0]=1;
    dp[1]=-1;
    for i in 0..n {
        if i>0 {
            dp[i]+=dp[i-1];
            dp[i]%=m;
        }
        for (j,k) in section.iter() {
            if i+j<n {
                dp[i+j] +=dp[i];
                dp[i+j] %=m;
            }
            if i+k <n{
                dp[i+k] -=dp[i];
                dp[i+k] %=m;
            }
        }
    }

    println!("{}", (dp[n-1]+m)%m);

}
