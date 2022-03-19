#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        S1:String, S2:String, S3:String, S4:String,
    }
    let mut st = HashSet::<String>::new();
    st.insert(S1);
    st.insert(S2);
    st.insert(S3);
    st.insert(S4);
    if st.len() == 4 {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
