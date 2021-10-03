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
    priority: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(Debug)]
struct Tree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord + std::fmt::Display + Copy> Node<T> {
    fn new(key: T, pri: T) -> Self {
        Node {
            key: key,
            priority: pri,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

impl<T: Ord + std::fmt::Display + Copy> Tree<T> {
    fn insert(t: Tree<T>, key: T, pri: T) -> Tree<T> {
        match t.0 {
            None => Tree(Some(Box::new(Node::new(key, pri)))),
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => Tree(Some(node)),
                Ordering::Less => {
                    let left = Tree::insert(node.left, key, pri);
                    if node.priority < left.0.as_ref().unwrap().priority {
                        // println!("aaa");
                        Tree::right_rotate(Node {
                            key: node.key,
                            priority: node.priority,
                            left: left,
                            right: node.right,
                        })
                    } else {
                        Tree(Some(Box::new(Node {
                            key: node.key,
                            priority: node.priority,
                            left: left,
                            right: node.right,
                        })))
                    }
                }
                Ordering::Greater => {
                    // key < node.key
                    let right = Tree::insert(node.right, key, pri);
                    if node.priority < right.0.as_ref().unwrap().priority {
                        Tree::left_rotate(Node {
                            key: node.key,
                            priority: node.priority,
                            left: node.left,
                            right: right,
                        })
                    } else {
                        Tree(Some(Box::new(Node {
                            key: node.key,
                            priority: node.priority,
                            left: node.left,
                            right: right,
                        })))
                    }
                }
            },
        }
    }
    fn delete(t: Tree<T>, key: T) -> Tree<T> {
        match t.0 {
            None => Tree(None),
            Some(node) => match node.key.cmp(&key) {
                Ordering::Equal => Tree::_delete(*node, key),
                Ordering::Less => {
                    let right = Tree::delete(node.right, key);
                    Tree(Some(Box::new(Node {
                        key: node.key,
                        priority: node.priority,
                        left: node.left,
                        right: right,
                    })))
                }
                Ordering::Greater => {
                    let left = Tree::delete(node.left, key);
                    Tree(Some(Box::new(Node {
                        key: node.key,
                        priority: node.priority,
                        left: left,
                        right: node.right,
                    })))
                }
            },
        }
    }
    fn _delete(t: Node<T>, key: T) -> Tree<T> {
        match (t.left.0.as_ref(), t.right.0.as_ref()) {
            (None, None) => Tree(None),
            (Some(l), None) => Tree::delete(Tree::right_rotate(t), key),
            (None, Some(r)) => Tree::delete(Tree::left_rotate(t), key),
            (Some(l), Some(r)) => match l.priority.cmp(&r.priority) {
                Ordering::Less | Ordering::Equal => Tree::delete(Tree::left_rotate(t), key),
                Ordering::Greater => Tree::delete(Tree::right_rotate(t), key),
            },
        }
    }

    fn find(root: &Tree<T>, key: T) -> bool {
        let mut current = root;
        while let Some(ref node) = current.0 {
            match node.key.cmp(&key) {
                Ordering::Less => current = &node.right,
                Ordering::Greater => current = &node.left,
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
    fn right_rotate(t: Node<T>) -> Tree<T> {
        match t.left.0 {
            Some(left) => Tree(Some(Box::new(Node {
                key: left.key,
                priority: left.priority,
                left: left.left,
                right: Tree(Some(Box::new(Node {
                    key: t.key,
                    priority: t.priority,
                    left: left.right,
                    right: t.right,
                }))),
            }))),
            None => Tree(Some(Box::new(t))),
        }
    }

    fn left_rotate(t: Node<T>) -> Tree<T> {
        match t.right.0 {
            Some(right) => Tree(Some(Box::new(Node {
                key: right.key,
                priority: right.priority,
                left: Tree(Some(Box::new(Node {
                    key: t.key,
                    priority: t.priority,
                    left: t.left,
                    right: right.left,
                }))),
                right: right.right,
            }))),
            None => Tree(Some(Box::new(t))),
        }
    }

    fn preorder(node: &Tree<T>) {
        if node.0.is_some() {
            print!(" {}", node.0.as_ref().unwrap().key);
            Tree::preorder(&node.0.as_ref().unwrap().left);
            Tree::preorder(&node.0.as_ref().unwrap().right);
        }
    }

    fn inorder(node: &Tree<T>) {
        if node.0.is_some() {
            Tree::inorder(&node.0.as_ref().unwrap().left);
            print!(" {}", node.0.as_ref().unwrap().key);
            Tree::inorder(&node.0.as_ref().unwrap().right);
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
    let mut root = Tree(None);

    for _ in 0..n {
        sc.new_line();
        let op: &str = &sc.get::<String>();
        match op {
            "insert" => {
                let key: i64 = sc.get();
                let pri: i64 = sc.get();
                root = Tree::insert(root, key, pri);
            }
            "print" => {
                Tree::inorder(&root);
                println!();
                Tree::preorder(&root);
                println!();
            }
            "find" => {
                let val: i64 = sc.get();
                if Tree::find(&root, val) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
            "delete" => {
                let key: i64 = sc.get();
                root = Tree::delete(root, key);
            }
            _ => panic!(),
        }
    }
}
