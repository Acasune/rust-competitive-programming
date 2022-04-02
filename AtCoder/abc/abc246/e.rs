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
    let dyx = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    let mut visited = vec![vec![vec![inf; 4]; N]; N];
    let mut visited2 = vec![vec![vec![false; 4]; N]; N];
    for &(dy, dx) in &dyx {
        if ay + dy < 0 as i64 || ay + dy >= N as i64 || ax + dx < 0 as i64 || ax + dx >= N as i64 {
            continue;
        }
        let ny = (ay + dy) as usize;
        let nx = (ax + dx) as usize;
        if board[ny][nx] == '#' {
            continue;
        }
        let ndir = transform(dy, dx);
        que.push_back((ay, ax, dy, dx));
        visited[ay as usize][ax as usize][ndir as usize] = 1;
    }
    while let Some((y, x, pdy, pdx)) = que.pop_front() {
        let pdir = transform(pdy, pdx);
        if y as usize == by && x as usize == bx {
            println!("{}", visited[y as usize][x as usize][pdir as usize]);
            return;
        }
        if visited2[y as usize][x as usize][pdir as usize] {
            continue;
        }

        visited2[y as usize][x as usize][pdir as usize] = true;
        let mut pval = visited[y as usize][x as usize][pdir as usize];
        for &(ndy, ndx) in &dyx {
            if y + ndy < 0 as i64
                || y + ndy >= N as i64
                || x + ndx < 0 as i64
                || x + ndx >= N as i64
            {
                continue;
            }
            let ny = (y + ndy) as usize;
            let nx = (x + ndx) as usize;
            if board[ny][nx] == '#' {
                continue;
            }
            let ndir = transform(ndy, ndx);
            let mut val = pval;

            if ndir != pdir {
                val += 1;
            }
            if visited[ny][nx][ndir as usize] > val {
                visited[ny][nx][ndir as usize] = val;
                if ndir == pdir {
                    que.push_front((ny as i64, nx as i64, ndy, ndx));
                } else {
                    que.push_back((ny as i64, nx as i64, ndy, ndx));
                }
            }
        }
    }

    println!("{}", -1);
}

fn transform(dy: i64, dx: i64) -> i64 {
    if dy == 1 && dx == 1 {
        0
    } else if dy == -1 && dx == 1 {
        1
    } else if dy == 1 && dx == -1 {
        2
    } else {
        3
    }
}
