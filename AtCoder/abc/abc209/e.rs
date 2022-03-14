#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::ascii::AsciiExt;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: usize = usize::MAX / 10;
const md: i64 = 52;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        S:[Chars;N],
    }
    let mut prefixes = vec![];
    let mut suffixes = vec![];
    let mut edges = vec![vec![]; 52 * 52 * 52 + 52 * 52 + 52];
    for s in S {
        let mut prefix = 0;
        for i in 0..3 {
            if s[i].is_lowercase() {
                prefix = prefix * md + (s[i] as i64 - 'a' as i64) as i64
            } else {
                prefix = prefix * md + (s[i] as i64 - 'A' as i64 + 26) as i64
            }
        }

        let mut suffix = 0;
        for i in s.len() - 3..s.len() {
            if s[i].is_lowercase() {
                suffix = suffix * md + (s[i] as i64 - 'a' as i64) as i64
            } else {
                suffix = suffix * md + (s[i] as i64 - 'A' as i64 + 26) as i64
            }
        }
        edges[prefix as usize].push(suffix as usize);
        prefixes.push(prefix);
        suffixes.push(suffix);
    }
    let mut visited = vec![0; 52 * 52 * 52 + 52 * 52 + 52];
    // 0 -> Not Visited, 1->win, 2->loop  3 -> Lose

    for i in 0..edges.len() {
        if edges[i].len() == 0 {
            visited[i] = 3;
        }
    }
    for i in 0..edges.len() {
        for &e in &edges[i] {
            if visited[e] == 3 {
                visited[i] = 1;
            }
        }
    }

    for i in 0..N {
        let ans = dfs(suffixes[i] as usize, &mut visited, &edges);
        if ans == 1 {
            println!("{}", "Aoki");
        } else if ans == 3 {
            println!("{}", "Takahashi");
        } else {
            println!("{}", "Draw");
        }
    }
}

fn dfs(s: usize, visited: &mut Vec<usize>, edges: &Vec<Vec<usize>>) -> usize {
    if visited[s] != 0 {
        return visited[s];
    }
    if edges[s].len() == 0 {
        visited[s] = 3;
        return visited[s];
    }
    visited[s] = 2;
    let mut tmp = 0;
    for &e in &edges[s] {
        tmp = tmp.max(dfs(e, visited, edges));
    }
    if tmp == 3 {
        visited[s] = 1;
    } else if tmp == 2 {
        visited[s] = 2;
    } else {
        visited[s] = 3;
    }

    return visited[s];
}
