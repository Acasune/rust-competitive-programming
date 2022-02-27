#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
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
        N:usize,
        maze:[Chars;N],
    }
    let mut flg = false;
    for i in 0..N {
        let mut l = 0;
        let mut r = 0;
        let mut cnt = 0;
        let mut len: i64 = 0;
        while r < N {
            if maze[i][r] == '.' {
                cnt += 1;
                while cnt == 3 {
                    if maze[i][l] == '.' {
                        cnt -= 1;
                    }
                    l += 1;
                }
            }
            r += 1;
            len = len.max((r - l) as i64);

            if len >= 6 {
                flg = true;
            }
        }
    }
    for i in 0..N {
        let mut l = 0;
        let mut r = 0;
        let mut cnt = 0;
        let mut len: i64 = 0;
        while r < N {
            if maze[r][i] == '.' {
                cnt += 1;
                while cnt == 3 {
                    if maze[l][i] == '.' {
                        cnt -= 1;
                    }
                    l += 1;
                }
            }
            r += 1;
            len = len.max((r - l) as i64);

            if len >= 6 {
                flg = true;
            }
        }
    }
    for i in 0..N {
        // start at maze[i][0] maze[i+1][0]...
        let mut l = 0;
        let mut r = 0;
        let mut cnt = 0;
        let mut len: i64 = 0;
        while i + r < N {
            if maze[i + r][r] == '.' {
                cnt += 1;
                while cnt == 3 {
                    if maze[i + l][l] == '.' {
                        cnt -= 1;
                    }
                    l += 1;
                }
            }
            r += 1;
            len = len.max((r - l) as i64);

            if len >= 6 {
                flg = true;
            }
        }
    }
    for i in 0..N {
        // start at maze[0][i] maze[0][i+1]...
        let mut l = 0;
        let mut r = 0;
        let mut cnt = 0;
        let mut len: i64 = 0;
        while i + r < N {
            if maze[r][i + r] == '.' {
                cnt += 1;
                while cnt == 3 {
                    if maze[l][l + i] == '.' {
                        cnt -= 1;
                    }
                    l += 1;
                }
            }
            r += 1;
            len = len.max((r - l) as i64);

            if len >= 6 {
                flg = true;
            }
        }
    }
    for i in 0..N {
        // start at maze[i][0] maze[i+1][0]... right upper
        let mut l = 0;
        let mut r = 0;
        let mut cnt = 0;
        let mut len: i64 = 0;
        while i >= r {
            if maze[i - r][r] == '.' {
                cnt += 1;
                while cnt == 3 {
                    if maze[i - l][l] == '.' {
                        cnt -= 1;
                    }
                    l += 1;
                }
            }
            r += 1;
            len = len.max((r - l) as i64);
            if len >= 6 {
                flg = true;
            }
        }
    }
    for i in (0..N).rev() {
        // start at maze[N-1][0] maze[N-1][1]...upper right
        let mut l = 0;
        let mut r = 0;
        let mut cnt = 0;
        let mut len: i64 = 0;
        while i + r < N {
            if maze[i + r][r] == '.' {
                cnt += 1;
                while cnt == 3 {
                    if maze[i + l][l] == '.' {
                        cnt -= 1;
                    }
                    l += 1;
                }
            }
            r += 1;
            len = len.max((r - l) as i64);

            if len >= 6 {
                flg = true;
            }
        }
    }
    if flg {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
