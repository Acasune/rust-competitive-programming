#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char,i32,f32,f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,mut S:Bytes,
        Q:usize,
    }
    let mut vec = vec![PURQ::new(N,0i64,|a,b| a+b);26];

    for (i,&c) in S.iter().enumerate() {
        let c = (c-'a' as u8) as usize;
        vec[c].update(i, 1);
    }
    for _ in 0..Q {
        input! {
            a:usize
        }
        if a == 1 {
            input! {
                b:Usize1,
                c:char
            }
            let c = c as u8;
            let idx = (c -'a' as u8) as usize;
            let prev = (S[b] - 'a' as u8)as usize;
            if S[b] != c {
                vec[prev].update(b,0);
                vec[(c - 'a' as u8) as usize].update(b, 1);
                S[b] = c;
            }
        } else {
            input! {
                l:Usize1,
                r:Usize1
            }
            let mut ans = 0;
            for c in 0..26 {
                if vec[c].find(l, r+1) >0 {
                    ans +=1;
                }
            }
            println!("{}",ans);
        }
    }

}

#[derive(Clone)]
struct PURQ<T, F> {
    n: usize,
    data: Vec<T>,
    e: T,
    op: F,
}

impl<T, F> PURQ<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    fn new(n: usize, e: T, op: F) -> Self {
        let size = n.next_power_of_two();
        PURQ {
            n: size,
            data: vec![e.clone(); 2 * size],
            e: e,
            op: op,
        }
    }
    fn update(&mut self, pos: usize, v: T) {
        assert!(pos < self.n);
        let mut pos = pos + self.n;
        let data = &mut self.data;
        data[pos] = v;
        pos >>= 1;
        while pos > 0 {
            data[pos] = (self.op)(&data[2 * pos], &data[2 * pos + 1]);
            pos >>= 1;
        }
    }
    fn update_tmp(&mut self, pos: usize, v: T) {
        assert!(pos < self.n);
        self.data[pos + self.n] = v;
    }
    fn update_all(&mut self) {
        let data = &mut self.data;
        for k in (1..self.n).rev() {
            data[k] = (self.op)(&data[2 * k], &data[2 * k + 1]);
        }
    }
    fn find(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e.clone();
        }
        let mut p = self.e.clone();
        let mut q = self.e.clone();
        let mut l = l + self.n;
        let mut r = r + self.n;
        let data = &self.data;
        while l < r {
            if l & 1 == 1 {
                p = (self.op)(&p, &data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                q = (self.op)(&data[r], &q)
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&p, &q)
    }
}
