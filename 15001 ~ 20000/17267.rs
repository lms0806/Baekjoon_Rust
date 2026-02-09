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
    let (l, r) = (scan.token::<i64>(), scan.token::<i64>());

    let (mut sx, mut sy) = (0, 0);
    let mut arr = vec![vec![0; m]; n];

    for i in 0..n {
        let line = scan.line();
        for (j, c) in line.bytes().enumerate() {
            arr[i][j] = (c - b'0') as i32;

            if arr[i][j] == 2 {
                sx = i;
                sy = j;
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut best_left = vec![vec![i64::MAX; m]; n];

    queue.push_back((sx, sy));
    best_left[sx][sy] = 0;

    let dx = [1, -1, 0, 0];
    let dy = [0, 0, 1, -1];

    while let Some((x, y)) = queue.pop_front() {
        let cur_left = best_left[x][y];
        let used_right = (y as i64 - sy as i64) + cur_left;

        if cur_left > l || used_right > r {
            continue;
        }

        for i in 0..4 {
            let (nx, ny) = (x as i64 + dx[i], y as i64 + dy[i]);

            if nx < 0 || nx >= n as i64 || ny < 0 || ny >= m as i64 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if arr[nx][ny] == 1 {
                continue;
            }

            let next_left = cur_left + if i == 3 { 1 } else { 0 };
            let next_right = (ny as i64 - sy as i64) + next_left;

            if next_left > l || next_right > r {
                continue;
            }

            if best_left[nx][ny] > next_left {
                best_left[nx][ny] = next_left;
                if i == 2 {
                    queue.push_back((nx, ny));
                } else {
                    queue.push_front((nx, ny));
                }
            }
        }
    }

    let mut answer = 0;
    for i in 0..n {
        for j in 0..m {
            if best_left[i][j] != i64::MAX {
                if best_left[i][j] <= l && (j as i64 - sy as i64) + best_left[i][j] <= r {
                    answer += 1;
                }
            }
        }
    }
    write!(out, "{}", answer).unwrap();
}
