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
        N:usize,
        ay:i64,ax:i64,
        by:Usize1,bx:Usize1,
        board:[Chars;N],
    }
    let ax = ax - 1;
    let ay = ay - 1;
    let mut que = VecDeque::new();
    let dy = [1, -1, 1, -1];
    let dx = [1, 1, -1, -1];
    let mut visited = vec![vec![vec![inf; 5]; N]; N];
    que.push_back((ay, ax, 4));
    visited[ay as usize][ax as usize][4] = 0;

    while let Some((y, x, pdir)) = que.pop_front() {
        for i in 0..4 {
            let ndy = dy[i];
            let ndx = dx[i];
            let ny = y + ndy;
            let nx = x + ndx;
            if !(0 <= ny && ny < N as i64) {
                continue;
            }
            if !(0 <= nx && nx < N as i64) {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if board[ny][nx] == '#' {
                continue;
            }
            if pdir == i {
                if visited[ny][nx][i] > visited[y as usize][x as usize][pdir as usize] {
                    visited[ny][nx][i] = visited[y as usize][x as usize][pdir as usize];
                    que.push_front((ny as i64, nx as i64, i));
                }
            } else {
                if visited[ny][nx][i] > visited[y as usize][x as usize][pdir as usize] + 1 {
                    visited[ny][nx][i] = visited[y as usize][x as usize][pdir as usize] + 1;
                    que.push_back((ny as i64, nx as i64, i));
                }
            }
        }
    }
    let mut ans = inf;
    for i in 0..4 {
        ans = ans.min(visited[by][bx][i]);
    }
    if ans == inf {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
