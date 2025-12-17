use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut min_pq, mut max_pq) = (BinaryHeap::new(), BinaryHeap::new());
    let mut live_map = HashMap::new();
    loop {
        match scan.token::<usize>() {
            0 => break,
            1 => {
                let (k, p) = (scan.token::<i64>(), scan.token::<i64>());
                max_pq.push((p, k));
                min_pq.push((Reverse(p), Reverse(k)));
                live_map.insert((k, p), true);
            }
            2 => {
                writeln!(out, "{}", pop_max(&mut max_pq, &mut live_map)).unwrap();
            }
            3 => {
                writeln!(out, "{}", pop_min(&mut min_pq, &mut live_map)).unwrap();
            }
            _ => {}
        };
    }
}

fn pop_max(max_pq: &mut BinaryHeap<(i64, i64)>, live: &mut HashMap<(i64, i64), bool>) -> i64 {
    while let Some((p, k)) = max_pq.pop() {
        if live.remove(&(k, p)) == Some(true) {
            return k;
        }
    }
    0
}

fn pop_min(
    min_pq: &mut BinaryHeap<(Reverse<i64>, Reverse<i64>)>,
    live: &mut HashMap<(i64, i64), bool>,
) -> i64 {
    while let Some((Reverse(p), Reverse(k))) = min_pq.pop() {
        if live.remove(&(k, p)) == Some(true) {
            return k;
        }
    }
    0
}
