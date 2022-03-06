#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::marker::*;
use proconio::*;

fn main() {
    input! {
        S:Bytes,
        Q:usize,
        TK:[(usize,usize);Q],
    }
    let pop_count = |x: usize| -> usize {
        let mut x = x;
        let mut ret = 0;
        while x > 0 {
            ret += x % 2;
            x /= 2;
        }
        return ret;
    };

    for (t, mut k) in TK {
        k -= 1;
        let upper = k / usize::pow(2usize, (60usize).min(t) as u32);
        let lower: usize = k % usize::pow(2usize, (60usize).min(t) as u32);
        let ans = b'A' + (((S[upper] + pop_count(lower) as u8 - b'A') as usize + t) % 3) as u8;
        println!("{}", ans as char);
    }
}
