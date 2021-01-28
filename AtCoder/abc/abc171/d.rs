use std::{collections::HashMap, io::*, str::FromStr};

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
    sc.new_line();
    let v:Vec<i64> = sc.get_as_vec();
    let mut sum = v.iter().sum::<i64>();

    let mut mp:HashMap<i64,i64> = HashMap::new();
    for &e in v.iter(){
        if mp.contains_key(&e) {
            mp.insert(e, mp.get(&e).unwrap()+1);
        }
        else {
            mp.insert(e,1);
        }

    }

    sc.new_line();
    let q:i64 =sc.get();

    for i in 0..q {
        sc.new_line();
        let b:i64 =sc.get();
        let c:i64 =sc.get();
        match mp.get(&b) {
            Some(x) => {
                sum +=(-b + c) * mp.get(&b).unwrap();
                mp.insert(c, mp.get(&c).unwrap_or(&0)+ mp.get(&b).unwrap());
                mp.remove(&b);

            }
            None => {}
        }
        println!("{}", sum);
    }



}
