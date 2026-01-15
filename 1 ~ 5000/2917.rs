use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
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
    let arr = (0..n)
        .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut sx, mut sy, mut ex, mut ey) = (0, 0, 0, 0);
    for i in 0..n {
        for j in 0..m {
            if arr[i][j] == 'V' {
                sx = i;
                sy = j;
            } else if arr[i][j] == 'J' {
                ex = i;
                ey = j;
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut tree = vec![vec![-1; m]; n];
    let mut visited = vec![vec![false; m]; n];

    for i in 0..n {
        for j in 0..m {
            if arr[i][j] == '+' {
                queue.push_back((i as i64, j as i64));
                visited[i][j] = true;
                tree[i][j] = 0;
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (x + dx, y + dy);

            if nx < 0 || nx >= n as i64 || ny < 0 || ny >= m as i64 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if tree[nx][ny] == -1 {
                tree[nx][ny] = tree[x as usize][y as usize] + 1;
                queue.push_back((nx as i64, ny as i64));
            }
        }
    }

    let mut pq = BinaryHeap::new();
    let mut visited = vec![vec![false; m]; n];

    pq.push(Node {
        dist: tree[sx][sy],
        x: sx,
        y: sy,
    });
    visited[sx][sy] = true;

    while let Some(Node { dist, x, y }) = pq.pop() {
        if x == ex && y == ey {
            write!(out, "{}", dist).unwrap();
            return;
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (x as i64 + dx, y as i64 + dy);

            if nx < 0 || nx >= n as i64 || ny < 0 || ny >= m as i64 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if !visited[nx][ny] {
                visited[nx][ny] = true;
                pq.push(Node {
                    dist: dist.min(tree[nx][ny]),
                    x: nx,
                    y: ny,
                });
            }
        }
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    dist: i32,
    x: usize,
    y: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
