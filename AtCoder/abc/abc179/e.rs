use std::{cmp::{max, min}, io::*, str::FromStr, usize};

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
    let mut n:i64 = sc.get();
    let mut x:i64 = sc.get();
    let m:i64 = sc.get();
    let mut id = vec![-1 as i64; m as usize ];
    let mut v = Vec::<i64>::new();

    let mut len=0;
    let mut tot =0;

    while id[x as usize]==-1 {
        v.push(x);
        id[x as usize] = len;
        len+=1;
        tot+=x;
        x=x*x%m;
    }

    let mut c = len-id[x as usize];
    let mut s = 0;
    for i in id[x as usize]..len{
        s+=v[i as usize];
    }

    let mut ans =0;
    if n <= len {
        ans = v.iter().take((n) as usize).sum::<i64>();
    } else {
        ans += tot;
        n -= len;
        ans += s * (n/c);
        n %= c;
        for i in 0..n {
            ans += v[id[x as usize] as usize +i as usize];
        }
    }
    println!("{}", ans);


}
