use std::{cmp::{max, min}, io::*, str::{Chars, FromStr}};

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
    let mut n=sc.get::<usize>();
    let mut k=sc.get::<usize>();
    let mut t= vec![Vec::<usize>::new();n];

    for i in 0..n {
        sc.new_line();
        let tt = sc.get_as_vec::<usize>();
        t[i] =tt;
    }
    let mut visited= vec![false;n];
    visited[0]=true;
    println!("{}", dfs(0,0,1,n,k,&t,visited));
}

fn dfs(s:usize,time:usize,cnt:usize,n:usize,k:usize,times:&Vec<Vec<usize>>, visited:Vec<bool>) -> i64{
    if cnt == n {
        return {
            if time + times[s][0]==k {
                1
            } else {
                0
            }
        }
    }
    let mut ans=0;

    for i in 0.. n {
        if visited[i]{
            continue;
        }
        else {
            let mut vis=visited.clone();
            vis[i]=true;
            let tm=times[s][i];
            ans += dfs(i,time + tm, cnt+1, n, k, times, vis);
        }
    }
    return ans;

}
