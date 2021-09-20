﻿#[allow(unused_imports)]
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
    val: i64,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

struct BinarySearchTree {
    head: Option<Rc<RefCell<Node>>>,
    n_node: usize,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree {
            head: None,
            n_node: 0,
        }
    }

    fn insert(&mut self, val: i64) {
        self.n_node += 1;
        if let None = self.head {
            self.head = Some(Rc::new(RefCell::new(Node {
                val: val,
                left: None,
                right: None,
            })))
        } else {
            let mut pa = self.head.clone().unwrap();
            loop {
                if val <= pa.borrow().val {
                    if pa.borrow().left.is_none() {
                        pa.borrow_mut().left = Some(Rc::new(RefCell::new(Node {
                            val: val,
                            left: None,
                            right: None,
                        })));
                        break;
                    } else {
                        let new_pa = pa.borrow().left.clone().unwrap();
                        pa = new_pa;
                    }
                } else {
                    if pa.borrow().right.is_none() {
                        pa.borrow_mut().right = Some(Rc::new(RefCell::new(Node {
                            val: val,
                            left: None,
                            right: None,
                        })));
                        break;
                    } else {
                        let new_pa = pa.borrow().right.clone().unwrap();
                        pa = new_pa;
                    }
                }
            }
        }
    }

    fn preorder(&self, node: Option<Rc<RefCell<Node>>>) {
        if node.is_none() {
            return;
        }
        print!(" {}", node.clone().unwrap().borrow().val);

        if node.clone().unwrap().borrow().left.is_some() {
            let new_node = node.clone().unwrap().borrow().left.clone();
            self.preorder(new_node);
        }
        if node.clone().unwrap().borrow().right.is_some() {
            let new_node = node.clone().unwrap().borrow().right.clone();
            self.preorder(new_node);
        }
    }

    fn inorder(&self, node: Option<Rc<RefCell<Node>>>) {
        if node.is_none() {
            return;
        }
        if node.clone().unwrap().borrow().left.is_some() {
            let new_node = node.clone().unwrap().borrow().left.clone();
            self.inorder(new_node);
        }
        print!(" {}", node.clone().unwrap().borrow().val);
        if node.clone().unwrap().borrow().right.is_some() {
            let new_node = node.clone().unwrap().borrow().right.clone();
            self.inorder(new_node);
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();

    let mut bst = BinarySearchTree::new();

    for _ in 0..n {
        sc.new_line();
        let op: &str = &sc.get::<String>();
        match op {
            "insert" => {
                let val: i64 = sc.get();
                bst.insert(val);
            }
            "print" => {
                bst.inorder(bst.head.clone());
                println!();
                bst.preorder(bst.head.clone());
                println!();
            }
            _ => panic!(),
        }
    }
}
