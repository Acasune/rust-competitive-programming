#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

fn main() {
    input! {
        T:usize,
        Q:[(usize,String);T],
    }

    let mut md: i64 = 998244353;
    let mut memo = vec![0; 1_000_100];
    calc(&mut memo, md);
    for (n, S) in Q {
        let cs = S.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = 0;
        let mut ans: i64 = 0;
        while j < (cs.len() + 1) / 2 {
            let mut cnt = cs[j] as i64 - 'A' as i64;
            cnt = cnt * memo[(n + 1) / 2 - i - 1];
            ans += cnt;
            i += 1;
            j += 1;
            ans %= md;
        }
        if n == 1 {
            ans += 1;
        } else {
            let mut l = 0;
            let mut r = 0;
            if n % 2 == 0 {
                l = n / 2 - 1;
                r = n / 2
            } else {
                l = n / 2 - 1;
                r = n / 2 + 1;
            }
            while r < n {
                if cs[r] < cs[l] {
                    break;
                } else if cs[r] > cs[l] {
                    ans += 1;
                    break;
                }
                r += 1;
                if r == n {
                    break;
                }
                l -= 1;
            }
            if r == n {
                ans += 1;
            }
        }

        println!("{}", ans % md);
    }
}

fn calc(memo: &mut Vec<i64>, md: i64) {
    memo[0] = 1;
    memo[1] = 26;
    for i in 2..memo.len() {
        memo[i] = memo[i - 1] * 26;
        memo[i] %= md;
    }
}
