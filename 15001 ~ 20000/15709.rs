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
            }
        }
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).expect("Failed read");

        if bytes_read == 0 {
            None
        } else {
            Some(input)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, b, k, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut dist = vec![vec![i64::MAX; n + m + b]; b];
    let mut arr = vec![Vec::<(usize, i64)>::new(); n + m + b];

    for _ in 0..k {
        let (a, bb, c) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<i64>(),
        );

        arr[a].push((bb, c));
        arr[bb].push((a, c));
    }

    for i in 0..b {
        let start = n + m + i;
        let mut pq = BinaryHeap::new();
        dist[i][start] = 0;
        pq.push(Reverse((0i64, start)));

        while let Some(Reverse((cost, now))) = pq.pop() {
            if cost > dist[i][now] {
                continue;
            }

            for &(next, w) in &arr[now] {
                if cost + w < dist[i][next] {
                    dist[i][next] = cost + w;
                    pq.push(Reverse((cost + w, next)));
                }
            }
        }
    }

    for _ in 0..q {
        let (a, bb) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

        let mut answer = i64::MAX;
        for i in 0..b {
            if dist[i][a] == i64::MAX || dist[i][bb] == i64::MAX {
                continue;
            }
            answer = answer.min(dist[i][a] + dist[i][bb]);
        }

        writeln!(out, "{}", if answer == i64::MAX { -1 } else { answer }).unwrap();
    }
}
