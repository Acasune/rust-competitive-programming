use std::{cmp::{max, min}, collections::VecDeque, io::*, str::{Chars, FromStr}};

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
    let mut h:usize =sc.get();
    let mut w:usize =sc.get();
    let mut k:i64 =sc.get();

    let mut field:Vec<Vec<char>> = Vec::new();

    for _ in 0..h {
        let mut tmp =sc.get_line();
        let tmp =tmp.chars().collect();
        field.push(tmp);
    }

    let mut ans =0;

    for h_mask in 0..(1<<h) {
        for w_mask in 0..(1<<w) {
            let mut field=field.clone();
            for i in 0..h {
                if 1<<i & h_mask ==0 {
                    continue
                }
                for j in 0..w{
                    field[i][j] ='r';
                }
            }
            for j in 0..w {
                if 1<<j & w_mask ==0 {
                    continue
                }
                for i in 0..h{
                    field[i][j] ='r';
                }
            }

            let cnt = count_black(h, w, field);
            ans = if cnt==k{
                ans +1
            } else {
                ans
            }
        }
    }

    println!("{}", ans);



}

fn count_black(h:usize, w:usize,v :Vec<Vec<char>>) -> i64{
    let mut cnt =0;
    for i in 0..h {
        for j in 0..w {
            if v[i][j]=='#'{
                cnt +=1;
            }
        }
    }

    return cnt;

}
