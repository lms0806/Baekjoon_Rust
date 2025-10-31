use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, s) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (mut left, mut right) = (vec![], vec![]);

    for _ in 0..n {
        let (idx, val) = (scan.token::<i64>(), scan.token::<i64>());

        if idx < s {
            left.push((idx, val));
        } else {
            right.push((idx, val));
        }
    }

    write!(out, "{}", solve(left, k, s) + solve(right, k, s)).unwrap();
}

fn solve(arr: Vec<(i64, i64)>, k: i64, s: i64) -> i64 {
    let mut dist = arr
        .iter()
        .map(|&(idx, val)| ((idx - s).abs(), val))
        .collect::<Vec<(i64, i64)>>();

    dist.sort_unstable_by_key(|&(a, _)| -a);

    let mut idx = 0;
    let mut answer = 0;
    while idx < dist.len() {
        answer += dist[idx].0 * 2;

        let mut num = k;
        while num > 0 && idx < dist.len() {
            if dist[idx].1 <= num {
                num -= dist[idx].1;
                dist[idx].1 = 0;
                idx += 1;
            } else {
                dist[idx].1 -= num;
                num = 0;
            }
        }
    }
    answer
}
