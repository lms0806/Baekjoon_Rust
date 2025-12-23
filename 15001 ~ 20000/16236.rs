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
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let (mut sx, mut sy) = (0, 0);
    let mut board = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            board[i][j] = scan.token::<usize>();

            if board[i][j] == 9 {
                board[i][j] = 0;
                sx = i;
                sy = j;
            }
        }
    }

    let (mut size, mut count, mut answer) = (2, 0, 0);
    let (dx, dy) = ([1, 0, -1, 0], [0, 1, 0, -1]);
    loop {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; n]; n];

        queue.push_back((sx, sy, 0));
        visited[sx][sy] = true;

        let mut min = i64::MAX;
        let mut target = Vec::new();
        while let Some((x, y, dist)) = queue.pop_front() {
            if dist > min {
                continue;
            }

            if board[x][y] > 0 && board[x][y] < size {
                min = dist;
                target.push((x, y));
                continue;
            }

            for i in 0..4 {
                let nx = x as i32 + dx[i];
                let ny = y as i32 + dy[i];

                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if visited[nx][ny] || board[nx][ny] > size {
                    continue;
                }

                visited[nx][ny] = true;
                queue.push_back((nx, ny, dist + 1));
            }
        }

        if target.is_empty() {
            break;
        }

        target.sort_unstable();
        let (tx, ty) = target[0];

        answer += min;
        sx = tx;
        sy = ty;
        board[tx][ty] = 0;

        count += 1;

        if count == size {
            size += 1;
            count = 0;
        }
    }
    write!(out, "{}", answer).unwrap();
}
