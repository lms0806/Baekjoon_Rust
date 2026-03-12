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

    for _ in 0..scan.token::<usize>() {
        let n = scan.token::<usize>();

        let arr = (0..n)
            .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let (mut sx, mut sy, mut ex, mut ey) = (0, 0, 0, 0);
        for i in 0..n {
            for j in 0..n {
                if arr[i][j] == 'S' {
                    sx = i;
                    sy = j;
                } else if arr[i][j] == 'X' {
                    ex = i;
                    ey = j;
                }
            }
        }

        let mut queue = VecDeque::new();
        let mut visited = vec![vec![vec![false; 8]; n]; n];

        queue.push_back((sx, sy, 0usize));
        visited[sx][sy][0] = true;

        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];

        let mut answer = 3;
        while let Some((x, y, mask)) = queue.pop_front() {
            if x == ex && y == ey {
                answer = answer.min(mask.count_ones() as i64);
                continue;
            }

            for i in 0..4 {
                let nx = x as i64 + dx[i];
                let ny = y as i64 + dy[i];

                if nx < 0 || nx >= n as i64 || ny < 0 || ny >= n as i64 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                let new_mask = match arr[nx][ny] {
                    'R' => mask | 1,
                    'B' => mask | 2,
                    'G' => mask | 4,
                    _ => mask,
                };

                if !visited[nx][ny][new_mask] {
                    visited[nx][ny][new_mask] = true;
                    queue.push_back((nx, ny, new_mask));
                }
            }
        }

        writeln!(out, "{}", answer).unwrap();
    }
}
