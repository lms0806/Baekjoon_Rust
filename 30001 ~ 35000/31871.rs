use io::Write;
use std::{cmp::max, io, str};

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

    let mut dist = vec![vec![-1; n + 1]; n + 1];
    for _ in 0..m {
        let (u, v, d) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );
        dist[u][v] = max(dist[u][v], d);
    }

    let mut visited = vec![false; n + 1];
    visited[0] = true;

    let mut answer = -1;
    dfs(0, 0, 0, n, &dist, &mut visited, &mut answer);

    writeln!(out, "{}", answer).unwrap();
}

fn dfs(
    node: usize,
    depth: usize,
    sum: i64,
    n: usize,
    dist: &Vec<Vec<i64>>,
    visited: &mut Vec<bool>,
    answer: &mut i64,
) {
    if depth == n {
        if dist[node][0] != -1 {
            *answer = max(*answer, sum + dist[node][0]);
        }
        return;
    }

    for v in 1..=n {
        if !visited[v] && dist[node][v] != -1 {
            visited[v] = true;
            dfs(v, depth + 1, sum + dist[node][v], n, dist, visited, answer);
            visited[v] = false;
        }
    }
}
