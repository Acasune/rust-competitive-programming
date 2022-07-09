#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,
    }

    let mut pes = vec![];
    let mut cnt = HashMap::<usize, usize>::new();
    let mut contr = HashMap::<usize, HashSet<usize>>::new();
    for i in 0..N {
        input! {
            m:usize,
            pe:[(usize,usize);m],
        }
        pes.push(pe.clone());
        for (p, e) in pe {
            if let Some(&d) = cnt.get(&p) {
                if e > d {
                    cnt.insert(p, e);
                    let mut st = HashSet::new();
                    st.insert(i);
                    contr.insert(p, st);
                }
                if e == d {
                    contr.get_mut(&p).unwrap().insert(i);
                }
            } else {
                cnt.insert(p, e);
                let mut st = HashSet::new();
                st.insert(i);
                contr.insert(p, st);
            }
        }
    }
    let mut ans_st = HashSet::new();
    for i in 0..N {
        let mut vec = vec![];
        for &(p, e) in &pes[i] {
            if contr.get(&p).unwrap().contains(&i) && contr.get(&p).unwrap().len() == 1 {
                vec.push(p);
            }
        }
        ans_st.insert(vec);
    }
    let mut ans = ans_st.len();
    println!("{}", ans);
}
