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
        let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
        let mut arr = vec![vec!['.'; m]; n];

        let (mut sx, mut sy, mut ex, mut ey) = (0, 0, 0, 0);
        for i in 0..n {
            for (j, value) in scan.token::<String>().chars().enumerate() {
                arr[i][j] = value;
                if arr[i][j] == 'S' {
                    sx = i;
                    sy = j;
                } else if arr[i][j] == 'E' {
                    ex = i;
                    ey = j;
                }
            }
        }

        let (dx, dy) = ([-1, 0, 1, 0], [0, 1, 0, -1]);
        let start_dir = if sx == 0 {
            2
        } else if sx == m - 1 {
            0
        } else if sy == 0 {
            1
        } else {
            3
        };

        let solve_wall = |is_left: bool| -> usize {
            let (mut x, mut y) = (sx, sy);
            let mut dir = start_dir;
            let mut count = 1;

            while x != ex || y != ey {
                if is_left {
                    dir = (dir + 3) % 4;
                } else {
                    dir = (dir + 1) % 4;
                }

                loop {
                    let (nx, ny) = (x as i64 + dx[dir], y as i64 + dy[dir]);

                    if nx >= 0 && nx < n as i64 && ny >= 0 && ny < m as i64 {
                        if arr[nx as usize][ny as usize] != '#' {
                            break;
                        }
                    }

                    dir = if is_left {
                        (dir + 1) % 4
                    } else {
                        (dir + 3) % 4
                    };
                }

                x = (x as i64 + dx[dir]) as usize;
                y = (y as i64 + dy[dir]) as usize;
                count += 1;
            }
            count
        };

        let bfs = || -> usize {
            let mut q = VecDeque::new();
            let mut dist = vec![vec![0; m]; n];

            dist[sx][sy] = 1;
            q.push_back((sx, sy));

            while let Some((x, y)) = q.pop_front() {
                if x == ex && y == ey {
                    return dist[x][y];
                }

                for i in 0..4 {
                    let (nx, ny) = (x as i64 + dx[i], y as i64 + dy[i]);

                    if nx >= 0 && nx < n as i64 && ny >= 0 && ny < m as i64 {
                        let (nx, ny) = (nx as usize, ny as usize);

                        if arr[nx][ny] != '#' && dist[nx][ny] == 0 {
                            dist[nx][ny] = dist[x][y] + 1;
                            q.push_back((nx, ny));
                        }
                    }
                }
            }
            0
        };

        writeln!(out, "{} {} {}", solve_wall(true), solve_wall(false), bfs()).unwrap();
    }
}
