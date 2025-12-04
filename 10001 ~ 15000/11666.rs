use std::cmp::Reverse;
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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut arr = (0..n)
        .map(|_| (scan.token::<i64>(), scan.token::<i64>()))
        .collect::<Vec<_>>();

    arr.sort_unstable();

    let mut pq = BinaryHeap::new();
    pq.push(Reverse(arr[0].0 + arr[0].1));

    let mut answer = 0;
    for i in 1..n {
        while let Some(&Reverse(top)) = pq.peek() {
            if arr[i].0 < top {
                break;
            }

            pq.pop();
            if top + m >= arr[i].0 {
                answer += 1;
                break;
            }
        }

        pq.push(Reverse(arr[i].0 + arr[i].1));
    }
    write!(out, "{}", answer).unwrap();
}
