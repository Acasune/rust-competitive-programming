use std::{cmp::min, io::*, str::{Chars, FromStr}};

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
    let mut s:Vec<char>=sc.get::<String>().chars().collect();

    let mut v= vec![0;10];

    if s.len()==1{
        if s[0].to_digit(10).unwrap()%8==0 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }
    if s.len()==2{
        let mut tmp1=s[0].to_digit(10).unwrap()*10 + s[1].to_digit(10).unwrap();
        let mut tmp2=s[1].to_digit(10).unwrap()*10 + s[0].to_digit(10).unwrap();
        if tmp1%8==0||tmp2%8==0 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    for &i in s.iter() {
        v[i.to_digit(10).unwrap() as usize]+=1;
    }

    let mut i =1;
    while i*8<1000 {
        let mut j = i*8;
        if j<100{
            j*=10;
        }else if j<10{
            j*=100
        }
        let mut v2=v.clone();
        i+=1;
        let mut flg=true;
        while j>0 {
            if v2[j%10 as usize]==0{
                flg =false;
                break;
            }
            v2[j%10 as usize]-=1;
            j=j/10;
        }
        if !flg {
            continue;
        }
        println!("Yes");
        return;
    }

    println!("No");

}
