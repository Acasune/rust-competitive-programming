#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        s:[Chars;N],
        t:[Chars;N],
    }
    let mut S = vec![vec!['a'; N]; N];
    let mut T = vec![vec!['a'; N]; N];
    for i in 0..N {
        for j in 0..N {
            S[i][j] = s[i][j];
            T[i][j] = t[i][j];
        }
    }
    let st = get_edge(&S, &T, N);
    let ok = check(&S, &T, N, st);
    // println!("{:?}", S);
    if ok {
        println!("{}", "Yes");
        return;
    }
    let S = right_rotate(&S, N);
    let st = get_edge(&S, &T, N);
    let ok = check(&S, &T, N, st);
    // println!("{:?}", S);
    if ok {
        println!("{}", "Yes");
        return;
    }

    let S = right_rotate(&S, N);
    let st = get_edge(&S, &T, N);
    let ok = check(&S, &T, N, st);
    // println!("{:?}", S);
    if ok {
        println!("{}", "Yes");
        return;
    }
    // println!("{:?}", S);
    let S = right_rotate(&S, N);
    let st = get_edge(&S, &T, N);
    let ok = check(&S, &T, N, st);
    // println!("{:?}", S);
    if ok {
        println!("{}", "Yes");
        return;
    }
    println!("{}", "No");
}
fn right_rotate(S: &Vec<Vec<char>>, N: usize) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; N]; N];
    for j in 0..N {
        for i in 0..N {
            ret[i][N - j - 1] = S[j][i];
        }
    }
    return ret;
}
fn check(
    S: &Vec<Vec<char>>,
    T: &Vec<Vec<char>>,
    N: usize,
    st: (usize, usize, usize, usize),
) -> bool {
    let mut s = vec![vec!['.'; N]; N];
    let mut t = vec![vec!['.'; N]; N];
    for j in 0..N {
        for i in 0..N {
            if S[j][i] == '#' {
                s[j - st.0][i - st.1] = '#';
            }
            if T[j][i] == '#' {
                t[j - st.2][i - st.3] = '#';
            }
        }
    }
    for j in 0..N {
        for i in 0..N {
            if s[j][i] != t[j][i] {
                return false;
            }
        }
    }
    return true;
}
fn get_edge(S: &Vec<Vec<char>>, T: &Vec<Vec<char>>, N: usize) -> (usize, usize, usize, usize) {
    let mut su = 0;
    let mut sl = 0;
    let mut tu = 0;
    let mut tl = 0;
    'outer: for j in 0..N {
        for i in 0..N {
            if S[j][i] == '#' {
                su = j;
                break 'outer;
            }
        }
    }
    'outer: for i in 0..N {
        for j in 0..N {
            if S[j][i] == '#' {
                sl = i;
                break 'outer;
            }
        }
    }
    'outer: for j in 0..N {
        for i in 0..N {
            if T[j][i] == '#' {
                tu = j;
                break 'outer;
            }
        }
    }
    'outer: for i in 0..N {
        for j in 0..N {
            if T[j][i] == '#' {
                tl = i;
                break 'outer;
            }
        }
    }
    (su, sl, tu, tl)
}
