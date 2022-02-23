#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

fn main() {
    input! {
        N:usize,M:usize,
        ed:[[usize;2];M],
    }

    let mut edges = vec![Vec::<usize>::new(); N + 1];
    for e in ed {
        edges[e[0] - 1].push(e[1] - 1);
        edges[e[1] - 1].push(e[0] - 1);
    }
    let mut max_len = 0;
    for i in 1..=N {
        max_len = max_len.max(edges[i].len());
    }

    let mut vs = Vec::<Vec<usize>>::new();
    let mut que = VecDeque::<usize>::new();
    let mut visited = vec![false; N + 1];
    for i in 0..N {
        if !visited[i] {
            let mut v = Vec::<usize>::new();
            que.push_back(i);
            while !que.is_empty() {
                let s = que.pop_back().unwrap();
                if visited[s] {
                    continue;
                }
                v.push(s);
                visited[s] = true;
                for &e in &edges[s] {
                    que.push_back(e);
                }
            }
            vs.push(v);
        }
    }
    let mut ans = 1i64;
    for v in &vs {
        let mut memo = vec![3; N + 10];
        let mut visited = vec![21; N + 10];
        memo[v[0]] = 0;
        visited[v[0]] = 0;
        let tmp = dfs(v[0], &edges, v, &mut memo, &mut visited) * 3;
        ans *= tmp;
    }

    println!("{}", ans);
}
fn dfs(
    i: usize,
    edges: &Vec<Vec<usize>>,
    v: &Vec<usize>,
    memo: &mut Vec<usize>,
    visited: &mut Vec<i64>,
) -> i64 {
    for &e in &edges[i] {
        if memo[i] == memo[e] {
            return 0;
        }
    }
    let mut ret = 1;
    for &e in &edges[i] {
        let mut tmp = 0;
        if visited[e] == 21 {
            visited[e] = i as i64 + 1;
        }
        if visited[e] != i as i64 + 1 {
            continue;
        }

        memo[e] = (memo[i] + 1) % 3;
        tmp += dfs(e, &edges, v, memo, visited);
        memo[e] = (memo[i] + 2) % 3;
        tmp += dfs(e, &edges, v, memo, visited);

        memo[e] = 3;

        ret *= tmp;
        if tmp == 0 {
            break;
        }
    }

    return ret;
}
