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
            };
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let mut arr = (0..n)
        .map(|_| {
            let (c1, t1) = (scan.token::<i64>(), scan.token::<i64>());
            (t1, c1)
        })
        .collect::<Vec<_>>();

    arr.sort_unstable_by_key(|x| x.0);

    let mut pq = BinaryHeap::new();
    let mut answer = 0;

    for (t1, c1) in arr {
        if pq.len() < (t1 + 1) as usize && pq.len() < t as usize {
            answer += c1;
            pq.push(Reverse(c1));
        } else if let Some(Reverse(min_c)) = pq.peek() {
            if *min_c < c1 {
                answer -= min_c;
                answer += c1;
                pq.pop();
                pq.push(Reverse(c1));
            }
        }
    }
    write!(out, "{}", answer).unwrap();
}
