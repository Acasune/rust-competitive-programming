#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
       maze:[[usize;4];4],
    }
    let mut s = (5, 5);
    let mut n = 0;
    for i in 0..4 {
        for j in 0..4 {
            if maze[j][i] == 1 {
                if s.0 == 5 {
                    s = (j, i);
                }
                n += 1;
            }
        }
    }
    let mut ans = 0;
    for bit in 0..(1 << 16) {
        let mut cand = vec![vec![0; 4]; 4];
        for i in 0..16 {
            if (bit >> i) & 1 == 1 {
                cand[i / 4][i % 4] = 1;
            }
        }

        if cand[s.0][s.1] == 0 {
            continue;
        }
        let mut visited = vec![vec![false; 4]; 4];
        let mut que = VecDeque::<(usize, usize)>::new();
        for j in 0..4 {
            for i in 0..4 {
                if (i == 0 || i == 3 || j == 0 || j == 3) && cand[j][i] == 0 {
                    visited[j][i] = true;
                    que.push_back((j, i));
                }
            }
        }
        while let Some((y, x)) = que.pop_front() {
            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)].iter() {
                let ny = y as i64 + dy;
                let nx = x as i64 + dx;
                if 0 <= ny && ny < 4 && 0 <= nx && nx < 4 {
                    if !visited[ny as usize][nx as usize] && cand[ny as usize][nx as usize] == 0 {
                        visited[ny as usize][nx as usize] = true;
                        que.push_back((ny as usize, nx as usize));
                    }
                }
            }
        }
        let mut flg = true;
        for j in 0..4 {
            for i in 0..4 {
                if cand[j][i] == 0 && !visited[j][i] {
                    flg = false;
                }
            }
        }
        if !flg {
            continue;
        }

        let mut visited = vec![vec![false; 4]; 4];
        let mut que = VecDeque::<(usize, usize)>::new();
        visited[s.0][s.1] = true;
        que.push_back(s);
        let mut cnt = 0;
        while let Some((y, x)) = que.pop_front() {
            if maze[y][x] == 1 {
                cnt += 1;
            }
            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)].iter() {
                let ny = y as i64 + dy;
                let nx = x as i64 + dx;
                if 0 <= ny && ny < 4 && 0 <= nx && nx < 4 {
                    if !visited[ny as usize][nx as usize] && cand[ny as usize][nx as usize] == 1 {
                        visited[ny as usize][nx as usize] = true;
                        que.push_back((ny as usize, nx as usize));
                    }
                }
            }
        }
        let mut flg = true;
        for j in 0..4 {
            for i in 0..4 {
                if cand[j][i] == 1 && !visited[j][i] {
                    flg = false;
                }
            }
        }
        if cnt == n && flg {
            ans += 1;
        }
    }
    println!("{}", ans);
}
