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
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, tmax) = (scan.token::<usize>(), scan.token::<i64>());
    let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    let mut answer = n;
    let (mut l, mut r) = (1, n);
    while l <= r {
        let mid = (l + r) >> 1;

        if check(&arr, mid, tmax) {
            answer = mid;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    writeln!(out, "{}", answer).unwrap();
}

fn check(arr: &Vec<i64>, k: usize, tmax: i64) -> bool {
    let mut pq = (0..k)
        .map(|idx| Reverse(arr[idx]))
        .collect::<BinaryHeap<_>>();

    for i in k..arr.len() {
        let time = pq.pop().unwrap().0 + arr[i];

        if time > tmax {
            return false;
        }

        pq.push(Reverse(time));
    }

    let mut last = 0;
    while let Some(Reverse(x)) = pq.pop() {
        last = last.max(x);
    }

    last <= tmax
}
