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
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BinarySearchTree {
    head: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { head: None }
    }

    fn insert(&mut self, val: i64) {
        let new_node = Box::new(Node {
            val: val,
            left: None,
            right: None,
        });
        if self.head.is_some() {
            let mut pa = &mut self.head;
            while let Some(v) = pa {
                if new_node.val <= v.val {
                    pa = &mut v.left;
                } else {
                    pa = &mut v.right;
                }
            }
            *pa = Some(new_node);
        } else {
            self.head = Some(Box::new(Node {
                val: val,
                left: None,
                right: None,
            }))
        }
    }
    fn find(&self, val: i64) -> bool {
        if self.head.is_some() {
            let mut pa = &self.head;
            while let Some(v) = pa {
                if v.val == val {
                    return true;
                } else if val < v.val {
                    pa = &v.left;
                } else {
                    pa = &v.right;
                }
            }
        }
        return false;
    }

    fn preorder(&self, node: &Option<Box<Node>>) {
        if let Some(ref raw_node) = node {
            print!(" {}", raw_node.val);
            self.preorder(&raw_node.left);
            self.preorder(&raw_node.right);
        }
    }

    fn inorder(&self, node: &Option<Box<Node>>) {
        if let Some(ref raw_node) = node {
            self.inorder(&raw_node.left);
            print!(" {}", raw_node.val);
            self.inorder(&raw_node.right);
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
                bst.inorder(&bst.head);
                println!();
                bst.preorder(&bst.head);
                println!();
            }
            "find" => {
                let val: i64 = sc.get();
                if bst.find(val) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
            _ => panic!(),
        }
    }
}
