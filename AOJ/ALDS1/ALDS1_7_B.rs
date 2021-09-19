#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
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
    fn new(reader: R) -> Scanner<R> {
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

struct Node {
    id: i64,
    parent: i64,
    sibling: i64,
    degree: i64,
    depth: i64,
    height: i64,
    left: i64,
    right: i64,
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let mut tree = Vec::<Node>::with_capacity(n);
    unsafe {
        tree.set_len(n);
    }

    for _ in 0..n {
        sc.new_line();
        let id = sc.get::<i64>();
        let l = sc.get::<i64>();
        let r = sc.get::<i64>();

        tree[id as usize] = Node {
            id: id,
            parent: -1,
            sibling: -1,
            degree: 0,
            depth: 0,
            height: 0,
            left: l,
            right: r,
        };
    }
    let mut que = VecDeque::<usize>::new();
    let mut que_leaves = VecDeque::<usize>::new();
    for i in 0..n {
        let parent_ = tree[i].id;
        let l = tree[i].left;
        let r = tree[i].right;
        if l != -1 {
            tree[l as usize].parent = parent_;
        }
        if r != -1 {
            tree[r as usize].parent = parent_;
        }
    }

    // search roots
    for node in &mut tree {
        if node.parent == -1 {
            que.push_back(node.id as usize);
            node.depth = 0;
        }
    }
    while !que.is_empty() {
        let target = que.pop_front().unwrap();
        let l = tree[target].left;
        let r = tree[target].right;
        if l == -1 && r == -1 {
            que_leaves.push_back(target);
        } else if l == -1 && r != -1 {
            tree[target].degree = 1;
            tree[r as usize].parent = target as i64;
            tree[r as usize].depth = tree[target].depth as i64 + 1;
            que.push_back(r as usize);
        } else if l != -1 && r == -1 {
            tree[target].degree = 1;
            tree[l as usize].parent = target as i64;
            tree[l as usize].depth = tree[target].depth as i64 + 1;
            que.push_back(l as usize);
        } else {
            tree[target].degree = 2;
            tree[l as usize].parent = target as i64;
            tree[l as usize].depth = tree[target].depth as i64 + 1;
            tree[l as usize].sibling = r;
            tree[r as usize].parent = target as i64;
            tree[r as usize].depth = tree[target].depth as i64 + 1;
            tree[r as usize].sibling = l;
            que.push_back(l as usize);
            que.push_back(r as usize);
        }
    }

    while !que_leaves.is_empty() {
        let target = que_leaves.pop_front().unwrap();
        let pa = tree[target].parent;
        if pa == -1 {
            continue;
        }
        tree[pa as usize].height = max(tree[pa as usize].height, tree[target].height + 1);
        que_leaves.push_back(pa as usize);
    }

    for node in tree {
        print!("node {}: ", node.id);
        print!("parent = {}, ", node.parent);
        print!("sibling = {}, ", node.sibling);
        print!("degree = {}, ", node.degree);
        print!("depth = {}, ", node.depth);
        print!("height = {}, ", node.height);
        if node.parent == -1 {
            print!("root");
        } else if node.degree == 0 {
            print!("leaf");
        } else {
            print!("internal node");
        }
        println!();
    }
}
