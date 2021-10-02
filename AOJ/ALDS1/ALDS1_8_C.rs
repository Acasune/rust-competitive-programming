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

use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    key: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(Debug)]
struct Tree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord + std::fmt::Display> {
    root: Tree<T>,
}

impl<T: Ord + std::fmt::Display> Node<T> {
    fn new(value: T) -> Self {
        Node {
            key: value,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

impl<T: Ord + std::fmt::Display> Tree<T> {
    fn insert(&mut self, value: T) {
        let mut current = self;
        while let Some(ref mut node) = current.0 {
            match node.key.cmp(&value) {
                Ordering::Less => current = &mut node.right,
                Ordering::Greater => current = &mut node.left,
                Ordering::Equal => current = &mut node.right,
            }
        }
        current.0 = Some(Box::new(Node::new(value)));
    }

    fn find(&mut self, value: T) -> bool {
        let mut current = self;
        while let Some(ref mut node) = current.0 {
            match node.key.cmp(&value) {
                Ordering::Less => current = &mut node.right,
                Ordering::Greater => current = &mut node.left,
                Ordering::Equal => return true,
            }
        }
        return false;
    }

    fn extract_min(&mut self) -> Option<T> {
        let mut node = None;

        if self.0.is_some() {
            let mut current = self;
            while current.0.as_ref().unwrap().left.0.is_some() {
                current = &mut current.0.as_mut().unwrap().left;
            }
            let tmp = current.0.take().unwrap();
            node = Some(tmp.key);
            current.0 = tmp.right.0
        }
        node
    }

    fn remove(&mut self, value: &T) {
        let mut current = self;
        while let Some(ref mut node) = current.0 {
            match node.key.cmp(value) {
                Ordering::Less => current = &mut current.0.as_mut().unwrap().right,
                Ordering::Greater => current = &mut current.0.as_mut().unwrap().left,
                Ordering::Equal => match (node.left.0.as_mut(), node.right.0.as_mut()) {
                    (None, None) => current.0 = None,
                    (Some(_), None) => current.0 = node.left.0.take(),
                    (None, Some(_)) => current.0 = node.right.0.take(),
                    (Some(_), Some(_)) => {
                        current.0.as_mut().unwrap().key = node.right.extract_min().unwrap();
                    }
                },
            }
        }
    }
    fn preorder(&self, node: &Tree<T>) {
        if node.0.is_some() {
            print!(" {}", node.0.as_ref().unwrap().key);
            self.preorder(&node.0.as_ref().unwrap().left);
            self.preorder(&node.0.as_ref().unwrap().right);
        }
    }

    fn inorder(&self, node: &Tree<T>) {
        if node.0.is_some() {
            self.inorder(&node.0.as_ref().unwrap().left);
            print!(" {}", node.0.as_ref().unwrap().key);
            self.inorder(&node.0.as_ref().unwrap().right);
        }
    }
}

impl<T: Ord + std::fmt::Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: Tree(None) }
    }
    pub fn find(&mut self, value: T) -> bool {
        return self.root.find(value);
    }
    pub fn insert(&mut self, value: T) {
        self.root.insert(value);
    }
    pub fn remove(&mut self, value: &T) {
        self.root.remove(value);
    }
    fn preorder(&self) {
        self.root.preorder(&self.root);
    }
    fn inorder(&self) {
        self.root.inorder(&self.root);
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();

    let mut bst = BinarySearchTree::<i64>::new();

    for _ in 0..n {
        sc.new_line();
        let op: &str = &sc.get::<String>();
        match op {
            "insert" => {
                let val: i64 = sc.get();
                bst.insert(val);
            }
            "print" => {
                bst.inorder();
                println!();
                bst.preorder();
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
            "delete" => {
                let val: i64 = sc.get();
                bst.remove(&val)
            }
            _ => panic!(),
        }
    }
}
