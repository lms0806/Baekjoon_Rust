use io::Write;
use std::collections::VecDeque;
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
    let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    write!(out, "{}", bfs(&arr, m)).unwrap();
}

fn bfs(arr: &[i64], m: i64) -> i64 {
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut visited = vec![false; 20001];

    visited[10000] = true;
    queue.push_back((0, 0));

    while !queue.is_empty() {
        let num = queue.pop_front().unwrap();

        if num.0 == m {
            return num.1;
        }

        for i in 0..arr.len() {
            if num.0 + arr[i] < -10000
                || num.0 + arr[i] > 10000
                || visited[(num.0 + arr[i] + 10000) as usize]
            {
                continue;
            }

            visited[(num.0 + arr[i] + 10000) as usize] = true;
            queue.push_back((num.0 + arr[i], num.1 + 1));
        }
    }

    -1
}
