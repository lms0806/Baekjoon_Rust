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
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..scan.token::<usize>() {
        let (n, m, t) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let (s, g, h) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
        );

        let mut arr = vec![vec![]; n];

        let mut weight = 0;
        for _ in 0..m {
            let (a, b, cost) = (
                scan.token::<usize>() - 1,
                scan.token::<usize>() - 1,
                scan.token::<i64>(),
            );

            arr[a].push((b, cost));
            arr[b].push((a, cost));

            if (a == g && b == h) || (a == h && b == g) {
                weight = cost;
            }
        }

        let mut target = (0..t)
            .map(|_| scan.token::<usize>() - 1)
            .collect::<Vec<_>>();

        target.sort_unstable();

        let (dist_s, dist_g, dist_h) = (dijkstra(s, &arr), dijkstra(g, &arr), dijkstra(h, &arr));

        for &x in &target {
            let (d1, d2) = (
                dist_s[g] + weight + dist_h[x],
                dist_s[h] + weight + dist_g[x],
            );

            if d1 == dist_s[x] || d2 == dist_s[x] {
                write!(out, "{} ", x + 1).unwrap();
            }
        }
        writeln!(out).unwrap();
    }
}

fn dijkstra(start: usize, arr: &[Vec<(usize, i64)>]) -> Vec<i64> {
    let mut dist = vec![i64::MAX; arr.len()];
    let mut pq = BinaryHeap::new();

    dist[start] = 0;
    pq.push((Reverse(0), start));

    while let Some((Reverse(cost), u)) = pq.pop() {
        if cost > dist[u] {
            continue;
        }

        for &(v, w) in &arr[u] {
            let nc = cost + w;
            if nc < dist[v] {
                dist[v] = nc;
                pq.push((Reverse(nc), v));
            }
        }
    }

    dist
}
