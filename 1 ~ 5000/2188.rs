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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut arr = vec![vec![]; n + 1];

    for i in 1..=n {
        for _ in 0..scan.token::<usize>() {
            arr[i].push(scan.token::<usize>());
        }
    }

    let mut result = vec![0; m + 1];

    let mut answer = 0;
    for i in 1..=n {
        let mut visited = vec![false; m + 1];
        if dfs(i, &arr, &mut result, &mut visited) {
            answer += 1;
        }
    }
    write!(out, "{}", answer).unwrap();
}

fn dfs(
    idx: usize,
    arr: &Vec<Vec<usize>>,
    result: &mut Vec<usize>,
    visited: &mut Vec<bool>,
) -> bool {
    for &book in &arr[idx] {
        if visited[book] {
            continue;
        }

        visited[book] = true;
        if result[book] == 0 || dfs(result[book], arr, result, visited) {
            result[book] = idx;
            return true;
        }
    }
    false
}
