use std::collections::VecDeque;
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

    let (m, n, s0) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let arr = (0..n)
        .map(|_| (scan.token::<i64>(), scan.token::<i64>()))
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    let mut visited = vec![false; m];

    queue.push_back((s0, 0));
    visited[s0 as usize] = true;

    while let Some(now) = queue.pop_front() {
        if now.0 == 0 {
            write!(out, "{}", now.1).unwrap();
            return;
        }

        for &(a, b) in &arr {
            let next = (now.0 * a + b) % m as i64;

            if !visited[next as usize] {
                visited[next as usize] = true;
                queue.push_back((next, now.1 + 1));
            }
        }
    }
    write!(out, "-1").unwrap();
}
