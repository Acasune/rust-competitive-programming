#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        H:usize, W:usize,
        maze:[Chars;H],
    }
    let mut visited = vec![vec![inf; W]; H];
    let mut que = VecDeque::<(i64, i64)>::new();
    que.push_back((0, 0));
    visited[0][0] = 0;
    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
            let ny = y + dy;
            let nx = x + dx;
            if 0 <= ny && ny < H as i64 && 0 <= nx && nx < W as i64 {
                if maze[ny as usize][nx as usize] == '.'
                    && visited[y as usize][x as usize] < visited[ny as usize][nx as usize]
                {
                    visited[ny as usize][nx as usize] = visited[y as usize][x as usize];
                    que.push_back((ny, nx));
                }
            }
        }
        for i in -2..=2 {
            for j in -2..=2 {
                if i64::abs(i) == 2 && i64::abs(j) == 2 {
                    continue;
                }
                let ny = y + i;
                let nx = x + j;
                if 0 <= ny && ny < H as i64 && 0 <= nx && nx < W as i64 {
                    if visited[y as usize][x as usize] + 1 < visited[ny as usize][nx as usize] {
                        visited[ny as usize][nx as usize] = visited[y as usize][x as usize] + 1;
                        que.push_back((ny, nx));
                    }
                }
            }
        }
    }
    println!("{}", visited[H - 1][W - 1]);
}
