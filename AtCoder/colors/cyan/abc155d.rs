#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, BTreeSet, VecDeque};
use std::convert::TryInto;
use std::{char,i32,f32,f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];



fn get_minus(m:i64,A:&Vec<i64>) -> i64 {
    let mut mis = vec![];
    let mut pls = vec![];
    for &a in A {
        if a < 0 {
            mis.push(a);
        } else if a > 0 {
            pls.push(a);
        }
    }
    let mut ret = 0;
    let mut j = 0;
    for mi in mis {
        while j < pls.len() && m< mi*pls[j] {
            j+=1;
        }
        ret += pls.len() as i64 - j as i64;
    }
    ret
}

fn get_zeros(A:&Vec<i64>) -> i64 {
    let mut zeros = 0;
    for &a in A {
        if a == 0 {
            zeros +=1;
        }
    }
    let ret = zeros *(A.len() as i64 - zeros) +zeros*(zeros-1)/2;
    ret
}

fn get_plus(m:i64,A:&Vec<i64>) -> i64 {
    let mut mis = vec![];
    let mut pls = vec![];
    for &a in A {
        if a >0 {
            pls.push(a);
        }
    }
    for i in (0..A.len()).rev() {
        let a = A[i];
        if a <0 {
            mis.push(-a);
        }
    }

    let mut ret = 0;
    let mut n = mis.len();
    let mut j = n-1;
    for i in 0..n {
        while i < j && m < mis[i] * mis[j] {
            j-=1;
        }
        ret += i64::max(0,j as i64 - i as i64);
    }

    n = pls.len();
    j = n-1;

    for i in 0..n {
        while i < j && m < pls[i] * pls[j] {
            j-=1;
        }
        ret += i64::max(0,j as i64 - i as i64);
    }

   
    ret
}


fn get_count(m:i64,A:&Vec<i64>) -> i64 {
    let mut ret = get_minus(m.min(-1),A);
    if 0 <= m {
        ret += get_zeros(A);
    }
    if 0 < m {
        ret += get_plus(m,A);
    }

    ret
}

fn main() {
    input! {
        N:usize,K:i64,
        mut A:[i64;N],
    }
    A.sort();
    let mut l = -inf_i*5;
    let mut r = inf_i*5;
    while r- l > 1 {
        let m = (r+l)/2;
        if get_count(m, &A) >=K {
            r = m;
        } else {
            l = m;
        }
        // println!("{}",m);
    }
    println!("{}",r);

}
