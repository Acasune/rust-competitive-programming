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
    id: usize,
    degree: usize,
    parent: usize,
    children: Vec<usize>,
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let guard = 1_000_000;

    sc.new_line();
    let n = sc.get::<usize>();
    let mut tree = Vec::<Node>::with_capacity(n);
    unsafe {
        tree.set_len(n);
    }

    for _ in 0..n {
        sc.new_line();
        let id = sc.get::<usize>();
        let k = sc.get::<usize>();
        let mut children = Vec::<usize>::new();
        for __ in 0..k {
            children.push(sc.get::<usize>());
        }
        tree[id] = Node {
            id: id,
            degree: guard,
            parent: guard,
            children: children,
        };
    }
    let mut que = VecDeque::<usize>::new();
    for i in 0..n {
        let parent_ = tree[i].id;
        let children_ = tree[i].children.clone();
        for c in children_ {
            tree[c].parent = parent_;
        }
    }
    for node in &mut tree {
        if node.parent == guard {
            que.push_back(node.id);
            node.degree = 0;
        }
    }
    while !que.is_empty() {
        let target = que.pop_front().unwrap();
        for c in tree[target].children.clone() {
            tree[c].degree = tree[target].degree + 1;
            que.push_back(c);
        }
    }
    for node in tree {
        print!("node {}: ", node.id);
        if node.parent == guard {
            print!("parent = {}, ", -1);
        } else {
            print!("parent = {}, ", node.parent);
        }
        print!("depth = {}, ", node.degree);
        if node.parent == guard {
            print!("root, ");
        } else if node.children.len() == 0 {
            print!("leaf, ");
        } else {
            print!("internal node, ");
        }
        if node.children.len() == 0 {
            println!("[]")
        } else {
            print!("[{}", node.children[0]);
            for i in 1..node.children.len() {
                print!(", {}", node.children[i]);
            }
            println!("]");
        }
    }
}
