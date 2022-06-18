#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, BTreeSet};
use std::convert::TryInto;
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
        N:usize,
        mut A:[i64;N],
    }
    let md = md998244353;
    let mut a_tmp = A.clone();
    a_tmp.sort();
    a_tmp.dedup();
    let mut mp = HashMap::new();
    for (i,a) in a_tmp.iter().enumerate() {
        mp.insert(*a,i as i64);
    }
    for i in 0..N {
        let a = A[i];
        A[i] = mp[&a] as i64  +1;
    }

    let div = mod_pow(2,md-2,md);
    let mut ans = 0;
    let mut bits = FenwickTree::new((mp[&*a_tmp.last().unwrap()] +2).try_into().unwrap(),0);
    for i in 0..N {
        let a = A[i];
        ans += bits.sum(a as usize +1, md) * mod_pow(2, i as i64, md);
        ans %= md;
        // println!("{}",ans);
        bits.add(a as usize, mod_pow(div, (i+1) as i64, md), md);
    }
    println!("{}",ans);


}


fn mod_pow(base: i64, power: i64, md: i64) -> i64 {
    // base^power % md
    let mut ret = 1;
    let mut base = base;
    let mut power = power;
    while power > 0 {
        if power % 2 == 1 {
            ret = (ret % md) * (base % md);
            ret %= md;
        }
        base = (base % md) * (base % md);
        base %= md;
        power >>= 1;
    }
    return ret;
}

struct FenwickTree<T> {
    n: usize,
    data: Vec<T>,
    e: T,
}

impl<T> FenwickTree<T>
where
    T: Clone,
    T: std::ops::AddAssign<T>,
{
    fn new(n: usize, e: T) -> Self {
        let size = n.next_power_of_two();
        FenwickTree {
            n: size,
            data: vec![e.clone(); size],
            e: e,
        }
    }
    fn addition(x:T,y:T,md:T) -> T
    where
    T: std::ops::Add<Output = T>,
    T: std::ops::Rem<Output = T>
    {
        (x+y)%md
    }
    fn add(&mut self, mut pos: usize, x: T,md:T)
    where
    T: std::ops::Add<Output = T>,
    T: std::ops::Rem<Output = T>,
    T: Copy
    {
        pos += 1;
        while pos <= self.n {
            self.data[pos - 1] = FenwickTree::addition(self.data[pos - 1].clone(),x.clone(),md);
            pos += pos & pos.wrapping_neg();
        }
    }
    fn sum(&self, mut pos: usize,md:T) -> T
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Rem<Output = T>,
        T: Copy
    {
        let data = &self.data;
        let mut s = self.e.clone();
        while pos > 0 {
            s = FenwickTree::addition(self.data[pos - 1].clone(),s,md);
            pos -= pos & pos.wrapping_neg();
        }
        s = FenwickTree::addition(self.data[pos].clone(),s,md);
        s
    }
}
