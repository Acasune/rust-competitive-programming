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
    let mut n=sc.get::<usize>();

    let mut x_l=Vec::<i64>::new();
    let mut y_l=Vec::<i64>::new();

    for _ in 0..n {
      sc.new_line();
      let x =sc.get::<i64>();
      let y =sc.get::<i64>();
      x_l.push(x);
      y_l.push(y);
    }

    for i in 0..n {
      for j in i+1..n {
        for k in j+1..n {
          let mut x1=x_l[i];
          let mut x2=x_l[j];
          let mut x3=x_l[k];

          let mut y1=y_l[i];
          let mut y2=y_l[j];
          let mut y3=y_l[k];
          
          x1-=x3;
          x2-=x3;
          y1-=y3;
          y2-=y3;

          if x1*y2==x2*y1 {
            println!("Yes");
            return;
          }
        }
      }
    }



    println!("No");


}
