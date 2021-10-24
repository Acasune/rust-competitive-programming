#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::BinaryHeap,
    collections::HashMap,
    collections::VecDeque,
    fmt::Display,
    rc::{Rc, Weak},
};
use std::{io::*, str::FromStr};

#[allow(unused_macros)]
macro_rules! scan {
  ($e:expr; $t:ty) => {
    $e.get::<$t>()
  };
  ($e:expr; $($t:ty), *) => {
    ($($e.get::<$t>(),)*)
  }
}

struct Scanner<R: BufRead> {
    reader: R,
    iter: std::vec::IntoIter<String>,
}

#[allow(dead_code)]
impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Scanner<R> {
        Scanner {
            reader,
            iter: vec![String::new()].into_iter(),
        }
    }
    fn new_line(&mut self) {
        let mut line = String::new();
        self.reader.read_line(&mut line).ok();
        self.iter = line
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter();
    }
    fn get<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
    fn get_as_vec<T: FromStr>(&mut self) -> Vec<T> {
        self.iter.clone().map(|v| v.parse().ok().unwrap()).collect()
    }
    fn get_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).ok();
        line.trim().to_string()
    }
}

struct State {
    cells: [u32; 16],
    pos: u32,
}

impl State {
    fn new() -> Self {
        State {
            cells: [0; 16],
            pos: 0,
        }
    }

    fn do_move(&mut self, op: Move) {
        let pos_next = match op {
            Move::Up => self.pos - 4,
            Move::Down => self.pos + 4,
            Move::Left => self.pos - 1,
            Move::Right => self.pos + 1,
        };
        self.cells.swap(self.pos as usize, pos_next as usize);
        self.pos = pos_next;
    }
    fn undo_move(&mut self, op: Move) {
        let pos_pre = match op {
            Move::Up => self.pos + 4,
            Move::Down => self.pos - 4,
            Move::Left => self.pos + 1,
            Move::Right => self.pos - 1,
        };
        self.cells.swap(self.pos as usize, pos_pre as usize);
        self.pos = pos_pre;
    }
    fn calc_diff(&self) -> u32 {
        return (0..16).fold(0, |dist, i| {
            dist + {
                let x = self.cells[i as usize];
                if x == 0 {
                    0
                } else {
                    State::calc_dist_per_cell(x - 1, i)
                }
            }
        });
    }

    fn calc_dist_per_cell(cell: u32, pos: u32) -> u32 {
        let py = pos / 4;
        let px = pos % 4;
        let cy = cell / 4;
        let cx = cell % 4;
        return if py > cy { py - cy } else { cy - py } + if px > cx { px - cx } else { cx - px };
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

impl Move {
    fn is_opposite(self, other: Self) -> bool {
        match (self, other) {
            (Self::Up, Self::Down)
            | (Self::Left, Self::Right)
            | (Self::Right, Self::Left)
            | (Self::Down, Self::Up) => true,
            _ => false,
        }
    }
    fn iter_from_pos(pos: u32) -> impl Iterator<Item = Self> {
        let u = if pos >= 4 { Some(Self::Up) } else { None };
        let d = if pos < 12 { Some(Self::Down) } else { None };
        let l = if pos % 4 >= 1 { Some(Self::Left) } else { None };
        let r = if pos % 4 < 3 { Some(Self::Right) } else { None };

        u.into_iter()
            .chain(d.into_iter())
            .chain(l.into_iter())
            .chain(r.into_iter())
    }
}

fn dfs(state: &mut State, mv_pre: Option<Move>, ans: &mut u32, cnt: u32) {
    let dist = state.calc_diff();
    if cnt + dist > *ans {
        return;
    }
    if dist == 0 {
        *ans = cnt;

        return;
    }
    for mv in Move::iter_from_pos(state.pos) {
        if let Some(mv_pre) = mv_pre {
            if mv.is_opposite(mv_pre) {
                continue;
            }
        }
        state.do_move(mv);
        dfs(state, Some(mv), ans, cnt + 1);
        state.undo_move(mv)
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let mut state = State::new();
    let mut i = 0;
    for _ in 0..4 {
        sc.new_line();
        let vv = sc.get_as_vec::<u32>();
        for v in vv {
            if v == 0 {
                state.pos = i as u32;
            }
            state.cells[i] = v;
            i += 1;
        }
    }
    let mut ans = 45;
    dfs(&mut state, None, &mut ans, 0);
    println!("{}", ans);
}
