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

const A:usize = 1_000_000;
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n:usize = sc.get();

    let mut fact=vec![0;A+1];

    sc.new_line();
    let v= sc.get_as_vec::<usize>();


    for &e in v.iter() {
        fact[e]+=1;
    }

    let mut pc_flag = true;

    for &p in get_primes().iter() {
        let mut c = 0;
        let mut cur = p;

        while cur <=A{
            c+=fact[cur];
            cur+=p;
        }
        if c>=2 {
            pc_flag=false;
            break;
        }

    }

    if pc_flag {
        println!("pairwise coprime");
        return
    }

    let mut g =0;
    for i in 0..n {
        g=gcd(g,v[i]);
    }
    if g==1 {
        println!("setwise coprime");
        return;
    }
    println!("not coprime");

}

fn get_primes() -> Vec<usize> {
    let mut is_prime = vec![true; A + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=A {
        if !is_prime[i] {
            continue;
        }
 
        let mut cur = i * 2;
        while cur <= A {
            is_prime[cur] = false;
            cur += i;
        }
    }
 
    let mut primes = vec![];
    for i in 2..=A {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
