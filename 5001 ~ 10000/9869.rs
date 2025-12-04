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

    let n = scan.token::<usize>();
    let mut arr = (0..n)
        .map(|_| (scan.token::<i64>(), scan.token::<i64>()))
        .collect::<Vec<_>>();

    arr.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut time = 0;
    let mut pq = BinaryHeap::new();
    for (milk, deadline) in arr {
        if time < deadline {
            pq.push(Reverse(milk));
            time += 1;
        } else {
            if let Some(&Reverse(min_milk)) = pq.peek() {
                if milk > min_milk {
                    pq.pop();
                    pq.push(Reverse(milk));
                }
            }
        }
    }

    write!(out, "{}", pq.iter().map(|Reverse(x)| x).sum::<i64>()).unwrap();
}
