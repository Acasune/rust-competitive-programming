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
    let mut visited = vec![vec![vec![inf; 4]; N]; N];
    let mut visited2 = vec![vec![vec![false; 4]; N]; N];
    for i in 0..4 {
        let dy = dy[i];
        let dx = dx[i];
        if !(0 <= ay + dy && ay + dy < N as i64) {
            continue;
        }
        if !(0 <= ax + dx && ax + dx < N as i64) {
            continue;
        }
        let ny = (ay + dy) as usize;
        let nx = (ax + dx) as usize;
        if board[ny][nx] == '#' {
            continue;
        }
        que.push_back((ay, ax, i));
        visited[ay as usize][ax as usize][i] = 1;
    }
    while let Some((y, x, pdir)) = que.pop_front() {
        if y as usize == by && x as usize == bx {
            println!("{}", visited[y as usize][x as usize][pdir as usize]);
            return;
        }
        if visited2[y as usize][x as usize][pdir as usize] {
            continue;
        }

        visited2[y as usize][x as usize][pdir as usize] = true;
        let mut pval = visited[y as usize][x as usize][pdir as usize];
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
            let ndir = i;
            let mut val = pval;

            if ndir != pdir {
                val += 1;
            }
            if visited[ny][nx][ndir as usize] > val {
                visited[ny][nx][ndir as usize] = val;
                if ndir == pdir {
                    que.push_front((ny as i64, nx as i64, ndir));
                } else {
                    que.push_back((ny as i64, nx as i64, ndir));
                }
            }
        }
    }

    println!("{}", -1);
}
