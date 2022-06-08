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
        H:usize,W:usize,K:usize,
        chocolate: [Chars;H],
    }
    let mut ans = inf_u;
    'outer: for bit in 0..(1<<H) {
        let mut mp = HashMap::new();
        let mut groups = vec![0;H];
        let mut g = 0;
        let prev = bit & 1;
        for i in 1..H {
            let b = (bit>>i) & 1;
            if prev != b {
                mp.insert(g,0);
                g+=1;
            }
            groups[i] = g;
        }
        mp.insert(g,0);
        g +=1;
        let mut cnt = g;
        for w in 0..W {
            let mut whites = vec![0;g];
            let mut need_divide = false;
            for h in 0..H {
                if chocolate[h][w] == '1' {
                    whites[groups[h]] +=1;
                }
            }
            for i in 0..g {
                if whites[i] + *mp.get(&i).unwrap() > K {
                    need_divide= true;
                }
            }
            if need_divide {
                cnt+=1;
                for i in 0..g {
                    if whites[i] >K {
                        continue 'outer;
                    }
                    mp.insert(i, whites[i]);
                }
            } else {
                for i in 0..g {
                    *mp.get_mut(&i).unwrap() += whites[i];
                }
            }
        }
        ans = ans.min(cnt);

    }
    println!("{}",ans-1);
}
