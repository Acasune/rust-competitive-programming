#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

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
        A:[usize;N],
        B:[usize;N],
        Q:usize,
        ask:[(usize,usize);Q],
    }
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut mp = HashMap::<usize, usize>::new();

    for &a in &A {
        mp.insert(a, rng.gen());
    }
    for &a in &B {
        mp.insert(a, rng.gen());
    }
    let mut avc = vec![0];
    let mut ast = HashSet::new();
    for a in A {
        let aa = avc[avc.len() - 1];
        if ast.contains(&a) {
            avc.push(aa);
        } else {
            avc.push(aa ^ mp.get(&a).unwrap());
            ast.insert(a);
        }
    }
    let mut bvc = vec![0];
    let mut bst = HashSet::new();
    for b in B {
        let bb = bvc[bvc.len() - 1];
        if bst.contains(&b) {
            bvc.push(bb);
        } else {
            bvc.push(bb ^ mp.get(&b).unwrap());
            bst.insert(b);
        }
    }
    for (x, y) in ask {
        if avc[x] == bvc[y] {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
