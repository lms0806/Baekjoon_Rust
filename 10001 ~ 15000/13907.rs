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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let (s, e) = (scan.token::<usize>(), scan.token::<usize>());

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

    let mut tax = vec![0; k + 1];
    for i in 1..=k {
        tax[i] = scan.token::<i64>();

        if i > 0 {
            tax[i] += tax[i - 1];
        }
    }

    let mut pq = BinaryHeap::new();
    let mut dist = vec![vec![i64::MAX; n + 1]; n + 1];

    pq.push(Node {
        idx: s,
        count: 0,
        cost: 0,
    });
    dist[s][0] = 0;

    while let Some(Node {
        idx: now,
        count,
        cost,
    }) = pq.pop()
    {
        if dist[now][count] < cost {
            continue;
        }

        if count >= n {
            continue;
        }

        for &(next, weight) in &arr[now] {
            if dist[next][count + 1] > cost + weight {
                dist[next][count + 1] = cost + weight;
                pq.push(Node {
                    idx: next,
                    cost: cost + weight,
                    count: count + 1,
                })
            }
        }
    }

    for i in 0..=k {
        let mut sum = i64::MAX;
        for j in 0..n {
            if dist[e][j] == i64::MAX {
                continue;
            }

            sum = sum.min(dist[e][j] + j as i64 * tax[i]);
        }
        writeln!(out, "{}", sum).unwrap();
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    idx: usize,
    count: usize,
    cost: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
