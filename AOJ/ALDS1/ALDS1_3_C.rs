#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
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
    value: i64,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

struct DoublyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.tail.is_none() && self.head.is_none()
    }

    fn get_head(&self) -> std::result::Result<i64, &'static str> {
        match &self.head {
            Some(head) => {
                let x = head.clone().borrow().value;
                Ok(x)
            }
            None => Err("Empty"),
        }
    }
    fn insert(&mut self, val: i64) {
        if let Some(head) = &self.head {
            let old_head = head.clone();
            let new_head = Rc::new(RefCell::new(Node {
                value: val,
                prev: None,
                next: Some(old_head.clone()),
            }));
            self.head = Some(new_head.clone());
            old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
            self.head = Some(new_head);
        } else {
            let elm = Rc::new(RefCell::new(Node {
                value: val,
                prev: None,
                next: None,
            }));
            self.head = Some(elm.clone());
            self.tail = Some(Rc::downgrade(&elm));
        }
    }

    fn delete(&mut self, val: i64) {
        match (&self.head.clone(), &self.tail) {
            (None, None) => {}
            (Some(head), Some(_)) => {
                let mut itr = Some(head.clone());
                while let Some(elm) = itr {
                    if elm.borrow().value == val {
                        self.delete_node(&elm.clone().borrow_mut());
                        return;
                    }
                    itr = elm.clone().borrow().next.clone();
                }
            }
            _ => panic!(),
        }
    }

    fn delete_first(&mut self) -> std::result::Result<(), &'static str> {
        match (&self.head.clone(), &self.tail) {
            (None, None) => Err("list is empty"),
            (Some(head), Some(_)) => {
                self.delete_node(&head.borrow_mut());
                Ok(())
            }
            _ => panic!(),
        }
    }

    fn delete_last(&mut self) -> std::result::Result<(), &'static str> {
        match (&self.head, &self.tail.clone()) {
            (None, None) => Err("list is empty"),
            (Some(_), Some(tail)) => {
                self.delete_node(&tail.upgrade().unwrap().borrow_mut());
                Ok(())
            }
            _ => panic!(),
        }
    }

    fn delete_node(&mut self, node: &std::cell::RefMut<Node>) {
        if let Some(prev) = node.prev.clone() {
            prev.upgrade().unwrap().borrow_mut().next = node.next.clone();
        } else {
            self.head = node.next.clone();
        }
        if let Some(next) = node.next.clone() {
            next.borrow_mut().prev = node.prev.clone();
        } else {
            self.tail = node.prev.clone();
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n: usize = sc.get();
    let mut list = DoublyLinkedList::new();
    for _ in 0..n {
        sc.new_line();
        let op: &str = &sc.get::<String>();
        match op {
            "insert" => {
                let x = sc.get::<i64>();
                list.insert(x);
            }
            "delete" => {
                let x = sc.get::<i64>();
                list.delete(x);
            }
            "deleteFirst" => {
                list.delete_first();
            }
            "deleteLast" => {
                list.delete_last();
            }
            _ => panic!(),
        }
    }
    let mut i = 0;
    while !list.is_empty() {
        if i != 0 {
            print!(" ");
        }
        i += 1;
        let x = list.get_head();
        list.delete_first().unwrap();
        print!("{}", x.unwrap());
    }
    println!();
}
