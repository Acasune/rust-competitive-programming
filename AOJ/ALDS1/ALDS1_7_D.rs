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
    pa: i64,
    left: i64,
    right: i64,
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

    let mut pre_map = HashMap::<i64, i64>::new();
    let mut in_map = HashMap::<i64, i64>::new();

    sc.new_line();
    let mut preorders: Vec<i64> = Vec::<i64>::new();
    for i in 0..n {
        let tmp = sc.get::<i64>() - 1;
        preorders.push(tmp);
        pre_map.insert(tmp, i as i64);
    }
    sc.new_line();
    let mut inorders: Vec<i64> = Vec::<i64>::new();
    for i in 0..n {
        let tmp = sc.get::<i64>() - 1;
        inorders.push(tmp);
        in_map.insert(tmp, i as i64);
    }
    let root = preorders[0];
    // construct tree
    let mut tree = Vec::<Node>::with_capacity(n);
    unsafe {
        tree.set_len(n);
    }
    for i in 0..n {
        tree[i] = Node {
            pa: -1,
            left: -1,
            right: -1,
        };
    }

    let mut pre_idx = 0;
    for i in 1..n {
        let mut pa = preorders[0];
        loop {
            if in_map.get(&(preorders[i] as i64)).unwrap() < in_map.get(&pa).unwrap() {
                if tree[pa as usize].left == -1 {
                    tree[pa as usize].left = preorders[i];
                    tree[preorders[i] as usize].pa = preorders[pa as usize];
                    break;
                } else {
                    pa = tree[pa as usize].left;
                }
            } else {
                if tree[pa as usize].right == -1 {
                    tree[pa as usize].right = preorders[i];
                    tree[preorders[i] as usize].pa = preorders[pa as usize];
                    break;
                } else {
                    pa = tree[pa as usize].right;
                }
            }
        }
    }
    let mut postorders = Vec::<i64>::new();
    postorder(&tree, preorders[0], &mut postorders);
    print!("{}", postorders[0] + 1);
    for i in 1..n {
        print!(" {}", postorders[i] + 1);
    }
    println!();
}
