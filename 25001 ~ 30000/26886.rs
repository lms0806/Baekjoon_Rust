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

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let arr = (0..2)
        .map(|_| (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; n]; 2];

    let mut answer = 0;
    visited[1][0] = true;
    dfs(n, k, 1, 1, 0, &arr, &mut visited, arr[1][0], &mut answer);
    write!(out, "{}", answer).unwrap();
}

fn dfs(
    n: usize,
    k: i64,
    count: i64,
    x: i64,
    y: i64,
    arr: &Vec<Vec<i64>>,
    visited: &mut Vec<Vec<bool>>,
    sum: i64,
    answer: &mut i64,
) {
    *answer = (*answer).max(sum);
    if count == k {
        return;
    }

    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (nx, ny) = (x + dx, y + dy);

        if nx >= 0 && nx < 2 && ny < n as i64 && ny >= 0 && !visited[nx as usize][ny as usize] {
            visited[nx as usize][ny as usize] = true;
            dfs(
                n,
                k,
                count + 1,
                x + dx,
                y + dy,
                arr,
                visited,
                sum + arr[nx as usize][ny as usize],
                answer,
            );
            visited[nx as usize][ny as usize] = false;
        }
    }
}
