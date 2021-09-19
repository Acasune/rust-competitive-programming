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
    left: i64,
    right: i64,
}

fn find_root(v: &Vec<Node>) -> i64 {
    let mut memo = vec![false; v.len()];
    for vv in v {
        let l = vv.left;
        let r = vv.right;
        if l != -1 {
            memo[l as usize] = true;
        }
        if r != -1 {
            memo[r as usize] = true;
        }
    }
    let mut ret = -1;
    for i in 0..memo.len() {
        if !memo[i] {
            ret = i as i64;
        }
    }
    return ret;
}

fn preorder(tree: &Vec<Node>, target: i64, sorted: &mut Vec<i64>) {
    sorted.push(target);
    if tree[target as usize].left != -1 {
        preorder(&tree, tree[target as usize].left, sorted);
    }
    if tree[target as usize].right != -1 {
        preorder(&tree, tree[target as usize].right, sorted);
    }
}

fn inorder(tree: &Vec<Node>, target: i64, sorted: &mut Vec<i64>) {
    if tree[target as usize].left != -1 {
        inorder(&tree, tree[target as usize].left, sorted);
    }
    sorted.push(target);
    if tree[target as usize].right != -1 {
        inorder(&tree, tree[target as usize].right, sorted);
    }
}

fn postorder(tree: &Vec<Node>, target: i64, sorted: &mut Vec<i64>) {
    if tree[target as usize].left != -1 {
        postorder(&tree, tree[target as usize].left, sorted);
    }
    if tree[target as usize].right != -1 {
        postorder(&tree, tree[target as usize].right, sorted);
    }
    sorted.push(target);
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

        tree[id as usize] = Node { left: l, right: r };
    }
    let root = find_root(&tree);
    let mut preorder_nodes = Vec::<i64>::new();
    let mut inorder_nodes = Vec::<i64>::new();
    let mut postorder_nodes = Vec::<i64>::new();
    preorder(&tree, root, &mut preorder_nodes);
    inorder(&tree, root, &mut inorder_nodes);
    postorder(&tree, root, &mut postorder_nodes);
    println!("Preorder");
    for i in 0..n {
        if i == n - 1 {
            println!(" {}", preorder_nodes[i]);
        } else {
            print!(" {}", preorder_nodes[i]);
        }
    }
    println!("Inorder");
    for i in 0..n {
        if i == n - 1 {
            println!(" {}", inorder_nodes[i]);
        } else {
            print!(" {}", inorder_nodes[i]);
        }
    }
    println!("Postorder");
    for i in 0..n {
        if i == n - 1 {
            println!(" {}", postorder_nodes[i]);
        } else {
            print!(" {}", postorder_nodes[i]);
        }
    }
}
