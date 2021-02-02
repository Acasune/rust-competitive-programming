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
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut n:Vec<char>=sc.get::<String>().chars().collect();
    let mut v =vec![0;3];

    let d_n = n.len();

    for c in n {
      v[(c.to_digit(10).unwrap()%3) as usize]+=1;
    }
    let mut ans =0;
    let sm:i64 = v[1]+2*v[2];
    if sm % 3 ==0 {
      println!("0");
      return ;
    }
    if sm % 3 ==1 {
      if v[1]>0 && d_n>1{
        println!("1");
        return ;
      }else if v[2]>1 &&d_n>2{
        println!("2");
        return ;
      }
    }
    if sm % 3 ==2 {
      if v[2]>0 && d_n>1{
        println!("1");
        return ;
      }else if v[1]>1 && d_n>2 {
        println!("2");
        return ;
      }
    }


    println!("-1");


}
