#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, BTreeSet};
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
        N:usize, Q:usize,
        AB: [(usize,Usize1);N],
        CD: [(Usize1,Usize1);Q],
    }
    let size = 2_00010;
    let mut max_rates = BTreeSet::new();
    let mut rates =vec![];
    let mut rate_sets =vec![BTreeSet::new();size];
    let mut groups = vec![];
    for  i in 0..N {
        let (a,b) = AB[i];
        rates.push(a);
        groups.push(b);
    }
    for i in 0..N {
        rate_sets[groups[i]].insert((rates[i], i));
    }

    for i in 0..size {
        if let Some(&(r, _)) = rate_sets[i].iter().last() {
            max_rates.insert((r, i));
        }
    }


    for (c,new_g) in CD {
        let cur_g = groups[c];
        groups[c] = new_g;

        if let Some(&(r,_)) = rate_sets[cur_g].iter().last() {
            max_rates.remove(&(r,cur_g));
        }
        if let Some(&(r,_)) = rate_sets[new_g].iter().last() {
            max_rates.remove(&(r,new_g));
        }
        rate_sets[cur_g].remove(&(rates[c],c));
        rate_sets[new_g].insert((rates[c],c));

        if let Some(&(r,_)) = rate_sets[cur_g].iter().last() {
            max_rates.insert((r,cur_g));
        }
        if let Some(&(r,_)) = rate_sets[new_g].iter().last() {
            max_rates.insert((r,new_g));
        }
        if let Some(&(r,_)) = max_rates.iter().next() {
            println!("{}",r);
        }
    }

}
