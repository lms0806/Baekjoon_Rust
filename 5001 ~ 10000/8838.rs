use std::collections::BinaryHeap;
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

    for _ in 0..scan.token::<usize>() {
        let (mut p, n) = (scan.token::<i64>(), scan.token::<usize>());

        let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
        let max = arr[n - 1];

        if p > max {
            writeln!(out, "1").unwrap();
            continue;
        }

        let mut pq = BinaryHeap::new();
        let (mut idx, mut answer) = (0, 0);

        loop {
            while idx < n && arr[idx] < p {
                pq.push(arr[idx]);
                idx += 1;
            }

            if pq.is_empty() {
                writeln!(out, "NIE").unwrap();
                break;
            }

            p += pq.pop().unwrap();
            answer += 1;

            if p > max {
                writeln!(out, "{}", answer + 1).unwrap();
                break;
            }
        }
    }
}
