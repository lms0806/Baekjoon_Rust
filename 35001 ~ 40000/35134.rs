use std::cmp::Ordering;
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
        let s = str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut arr = vec![vec![]; n + 1];

    for _ in 0..m {
        let (a, b, cost) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        arr[a].push((b, cost));
        arr[b].push((a, cost));
    }

    let dijkstra = |start: usize, weight_calc: &dyn Fn(i64) -> i64| -> Vec<i64> {
        let mut dist = vec![i64::MAX; n + 1];
        let mut pq = BinaryHeap::new();

        dist[start] = 0;
        pq.push(Node {
            idx: start,
            dist: 0,
        });

        while let Some(Node { idx, dist: d }) = pq.pop() {
            if dist[idx] < d {
                continue;
            }

            for &(next, cost) in &arr[idx] {
                let cost = weight_calc(cost);

                if dist[idx] + cost < dist[next] {
                    dist[next] = dist[idx] + cost;
                    pq.push(Node {
                        idx: next,
                        dist: dist[next],
                    });
                }
            }
        }

        dist
    };

    let (d1, d2, d3) = (
        dijkstra(1, &|x| x),
        dijkstra(2, &|x| x + (x + 1) / 2),
        dijkstra(3, &|x| x),
    );

    let mut answer = i64::MAX;

    for i in 1..=n {
        if d1[i] == i64::MAX || d2[i] == i64::MAX || d3[i] == i64::MAX {
            continue;
        }

        answer = answer.min(d1[i] + d2[i] + d3[i]);
    }
    write!(out, "{}", answer).unwrap()
}

#[derive(Eq, PartialEq)]
struct Node {
    idx: usize,
    dist: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
