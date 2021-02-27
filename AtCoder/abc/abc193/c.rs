use std::{
    cmp::{max, min},
    collections::HashSet,
    io::*,
    str::FromStr,
};
use std::{collections::hash_set, iter::FromIterator};

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
    let n = sc.get::<usize>();

    let mut hs = HashSet::new();

    for i in 2..n {
        if i * i > n {
            break;
        }
        // let mut tmp = i;

        // if hs.contains(&tmp) {
        //     hs.remove(&tmp);
        //     continue;
        // };

        let mut tmp = i * i;
        while tmp <= n {
            hs.insert(tmp);
            if tmp > n / i {
                break;
            }
            tmp *= i;
            // for k in 0..k {
            //     tmp *= i;
            //     if tmp > n {
            //         break;
            //     }
            // }
        }
    }
    let cnt = hs.len();

    println!("{}", n as i64 - cnt as i64);
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
        if i * i > n {
            return true;
        }
    }
    true
}

// pub fn prime_sieve(n: usize) -> i64 {
//     let mut cnt = 0;
//     // let m = f64::sqrt(n as f64) as usize;
//     let mut s = vec![0 as isize; n + 1];
//     s[0] = 1;
//     s[1] = 1;
//     for i in 2..n {
//         if i * i > n {
//             break;
//         }
//         let mut tmp = i;
//         if s[i] == 0 {
//             let mut k = 2;
//             tmp = i * i;
//             while tmp <= n {
//                 if s[tmp] == 0 {
//                     cnt += 1;
//                 }
//                 s[tmp] = 1;
//                 k += 1;
//                 if tmp > n / i {
//                     break;
//                 }
//                 tmp *= i;

//                 // for k in 0..k {
//                 //     tmp *= i;
//                 //     if tmp > n {
//                 //         break;
//                 //     }
//                 // }
//             }
//         }
//     }
//     cnt
// }
