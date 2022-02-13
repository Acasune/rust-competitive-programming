use std::{
    cmp::{max, min},
    io::*,
    str::FromStr,
};

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
    let N: usize = sc.get();
    let K: usize = sc.get();

    sc.new_line();
    let mut P = sc.get_as_vec::<usize>();
    for i in 0..N {
        P[i] -= 1;
    }
    sc.new_line();
    let C = sc.get_as_vec::<i64>();

    let mut ret = -1_000_000_000_000_000;

    for i in 0..N {
        let mut idxes = vec![0; N];
        let mut vals = vec![0; N];
        let mut idx_cnt = vec![0; N];
        let mut looped = false;
        let mut idx = i;
        let mut past_idx = i;
        let mut points = 0;
        let mut tmp_max = -1_000_000_000_000_000;
        let mut k = 1;
        loop {
            if k > K {
                break;
            }
            past_idx = idx;
            idx = P[idx];
            if idx_cnt[idx] != 0 && !looped {
                // cycled
                // cycle size: k - idx_cnt[idx]
                let cycle_size = k - idx_cnt[idx] as usize;
                let points_per_cycle = vals[past_idx] - vals[idx] + C[idx];
                // remain turns : K-k
                let cnt_cycle = (K - k) / (cycle_size);

                if points_per_cycle > 0 {
                    points += cnt_cycle as i64 * points_per_cycle;
                }

                k = k + cnt_cycle * cycle_size;
                looped = true;
            }
            points += C[idx];
            vals[idx] = points;
            tmp_max = tmp_max.max(points);
            idx_cnt[idx] = k as i64;
            k += 1;
        }
        ret = ret.max(tmp_max);
    }

    println!("{}", ret);
}
