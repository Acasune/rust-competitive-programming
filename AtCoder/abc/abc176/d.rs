use std::{cmp::{max, min}, io::*, str::FromStr};

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
    let n:usize = sc.get();

    sc.new_line();
    let mut v:Vec<char> = sc.get::<String>().chars().collect();

    let mut l = vec![0;n];
    let mut r = vec![0;n];

    l[0]=if v[0]=='W'{1} else {0};
    for i in 1..n {
        if v[i]=='W'{
            l[i] = l[i-1] + 1;
        } else {l[i] = l[i-1]};
    }
    r[n-1]=if v[n-1]=='R'{1} else {0};
    for i in (0..n-1).rev() {
        if v[i]=='R'{
            r[i] = r[i+1] + 1;
        } else {r[i] = r[i+1]};
    }
    let mut ans =1_000_000_010;

    if l[n-1]==n || r[0]==n {
        println!("0");
        return;
    }

    for i in 0..n-1 {
        let mut tmp;
        tmp = min(l[i], r[i+1]);
        tmp += max(l[i], r[i+1]) - min(l[i], r[i+1]);
        ans = min(ans, tmp);
    }

    println!("{}",ans);


}
