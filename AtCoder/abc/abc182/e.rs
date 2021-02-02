use std::{cmp::{max, min}, convert::TryInto, io::*, str::{Chars, FromStr}};

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

fn main() {

    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut h:usize= sc.get();
    let mut w:usize= sc.get();
    let mut n:usize= sc.get();
    let mut m:usize= sc.get();

    //a-> not lighted, b-> block, c->horizontal, d-> vertical

    let mut field = vec![vec!['a';w];h];

    let mut light = Vec::<Vec<usize>>::new();

    for i in 0..n {
        sc.new_line();
        let y:usize= sc.get();
        let x:usize= sc.get();
        light.push(vec![x-1,y-1]);
    }
    for j in 0..m {
        sc.new_line();
        let y:usize= sc.get();
        let x:usize= sc.get();
        field[y-1][x-1]='b';
    }

    for e in &light {
        let x =e[0];
        let y = e[1];
        if field[y][x]!='c' {
            field[y][x]='c';
            let mut tmp_x=x;
            loop {
                while tmp_x>0 {
                    tmp_x-=1;
                    if field[y][tmp_x]=='b'{
                        break;
                    } else {
                        field[y][tmp_x]='c';
                    }
                }
                break;
            }
            let mut tmp_x=x;
            loop {
                while tmp_x<w-1 {
                    tmp_x+=1;
                    if field[y][tmp_x]=='b'{
                        break;
                    } else {
                        field[y][tmp_x]='c';
                    }
                    
                }
                break;
            }
        }
    }

    for e in &light {
        let x =e[0];
        let y = e[1];
        if field[y][x]!='d' {
            let mut tmp_y=y;
            loop {
                
                while tmp_y>0 {
                    tmp_y-=1;
                    if field[tmp_y][x]=='b'{
                        break;
                    } else {
                        field[tmp_y][x]='d';
                    }
                }
                break;
            }
            let mut tmp_y=y;
            loop {
                while tmp_y<h-1 {
                    tmp_y+=1;
                    if field[tmp_y][x]=='b'{
                        break;
                    } else {
                        field[tmp_y][x]='d';
                    }
                }
                break;
            }
        }
    }

    let mut ans=0;
    for i in 0..h {
        for j in 0..w {
            if field[i][j]=='c'|| field[i][j]=='d'{
                ans+=1;
            }
        }
    }


 
    println!("{}",ans);

}
