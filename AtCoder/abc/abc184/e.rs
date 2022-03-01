#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
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
       H:usize,W:usize,
      maze:[Chars;H],
    }
    let inf: i64 = 1_000_000_000_000_000;
    let DYX = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = vec![vec![inf; W]; H];
    let mut s = (0, 0);
    let mut g = (0, 0);
    let mut seq = 0;
    let mut sets = vec![HashSet::new(); 26];
    for j in 0..H {
        for i in 0..W {
            if maze[j][i] == 'S' {
                s = (j, i);
            } else if maze[j][i] == 'G' {
                g = (j, i);
            } else if maze[j][i].is_alphabetic() {
                // println!("{}", maze[j][i] as usize - 'a' as usize);
                sets[maze[j][i] as usize - 'a' as usize].insert((j, i, seq));
                seq += 1;
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_back((s.0, s.1, 0));
    visited[s.0][s.1] = 0;
    let mut visited_alphabets = HashSet::<char>::new();
    while !que.is_empty() {
        let (y, x, cnt) = que.pop_front().unwrap();
        for &(dy, dx) in &DYX {
            let ny = y as i64 + dy;
            let nx = x as i64 + dx;
            if ny < 0 || H as i64 <= ny || nx < 0 || W as i64 <= nx {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if visited[ny][nx] <= cnt + 1 || maze[ny][nx] == '#' {
                continue;
            }

            visited[ny][nx] = cnt + 1;
            que.push_back((ny, nx, cnt + 1));
            if maze[ny][nx] != '#'
                && maze[ny][nx] != '.'
                && maze[ny][nx] != 'S'
                && maze[ny][nx] != 'G'
                && !visited_alphabets.contains(&maze[ny][nx])
            {
                visited_alphabets.insert(maze[ny][nx]);
                for &(h, w, _) in sets[maze[ny][nx] as usize - 'a' as usize].iter() {
                    if visited[h][w] > cnt + 2 {
                        visited[h][w] = cnt + 2;
                        que.push_back((h, w, cnt + 2));
                    }
                }
            }
        }
    }
    let mut ans = visited[g.0][g.1];

    if ans == inf {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
