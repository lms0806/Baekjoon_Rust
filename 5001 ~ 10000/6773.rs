use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::Write;
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
            buf_str: Vec::with_capacity(1 << 16),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    #[inline(always)]
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
            };
        }
    }

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok()?);
            }
            self.buf_str.clear();
            let n = self.reader.read_until(b'\n', &mut self.buf_str).ok()?;
            if n == 0 {
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_iter = "".split_ascii_whitespace();
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut arr = vec![vec![]; n + 1];
    let mut kpq = vec![BinaryHeap::new(); n + 1];
    let mut set = vec![HashSet::new(); n + 1];

    for _ in 0..m {
        arr[scan.token::<usize>()].push((scan.token::<usize>(), scan.token::<i64>()));
    }

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((1, 0)));
    kpq[1].push(0);

    while let Some(Reverse((end, cost))) = pq.pop() {
        for next in &arr[end] {
            let next_cost = next.1 + cost;

            if set[next.0].contains(&next_cost) {
                continue;
            }

            if kpq[next.0].len() < 2 {
                kpq[next.0].push(next_cost);
                set[next.0].insert(next_cost);
                pq.push(Reverse((next.0, next_cost)));
            } else if kpq[next.0].peek().unwrap() > &next_cost {
                kpq[next.0].pop();
                kpq[next.0].push(next_cost);
                set[next.0].insert(next_cost);
                pq.push(Reverse((next.0, next_cost)));
            }
        }
    }

    write!(
        out,
        "{}",
        if kpq[n].len() < 2 {
            -1
        } else {
            *kpq[n].peek().unwrap()
        }
    )
    .unwrap();
}
