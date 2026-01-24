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
    let mut arr = vec![vec![0u8; m]; n];

    let (mut sx, mut sy, mut ex, mut ey) = (0, 0, 0, 0);
    for i in 0..n {
        let s = scan.token::<String>();
        for (j, ch) in s.bytes().enumerate() {
            arr[i][j] = ch;

            if ch == b'C' {
                sx = i;
                sy = j;
                arr[i][j] = b'.';
            } else if ch == b'D' {
                ex = i;
                ey = j;
            }
        }
    }

    let fall = |x: usize, y: usize, g: usize, arr: &Vec<Vec<u8>>| -> (Option<usize>, bool) {
        let mut now = x as i64;
        let dx = if g == 0 { 1 } else { -1 };

        if x == ex && y == ey {
            return (Some(now as usize), true);
        }

        loop {
            let next = now + dx;

            if next < 0 || next >= arr.len() as i64 {
                return (None, false);
            }

            if arr[next as usize][y] == b'#' {
                return (Some(now as usize), false);
            }

            now = next;

            if now == ex as i64 && y == ey {
                return (Some(now as usize), true);
            }
        }
    };

    let mut moved = |x: usize,
                     y: usize,
                     g: usize,
                     cost: i64,
                     check: bool,
                     arr: &Vec<Vec<u8>>,
                     dist: &mut Vec<Vec<Vec<i64>>>,
                     queue: &mut VecDeque<(usize, usize, usize)>|
     -> bool {
        let (nx_op, hit) = fall(x, y, g, &arr);

        if hit {
            write!(out, "{}", cost).unwrap();
            return true;
        }

        if let Some(nx) = nx_op {
            if dist[nx][y][g] > cost {
                dist[nx][y][g] = cost;

                if check {
                    queue.push_front((nx, y, g));
                } else {
                    queue.push_back((nx, y, g));
                }
            }
        }
        false
    };

    let mut queue = VecDeque::new();
    let mut dist = vec![vec![vec![i64::MAX; 2]; m]; n];

    if moved(sx, sy, 0, 0, false, &arr, &mut dist, &mut queue) {
        return;
    }

    let dir = [-1, 1];
    while let Some((x, y, g)) = queue.pop_front() {
        let d = dist[x][y][g];

        if x == ex && y == ey {
            write!(out, "{}", d).unwrap();
            return;
        }

        for i in 0..2 {
            let ny = y as i64 + dir[i];

            if ny < 0 || ny >= m as i64 {
                continue;
            }

            if arr[x][ny as usize] == b'#' {
                continue;
            }

            if moved(x, ny as usize, g, d, true, &arr, &mut dist, &mut queue) {
                return;
            }
        }

        if moved(x, y, 1 - g, d + 1, false, &arr, &mut dist, &mut queue) {
            return;
        }
    }
    write!(out, "-1").unwrap();
}
