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

    let dir = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for t in 1..=scan.token::<usize>() {
        let n = scan.token::<usize>();

        let board = (0..n)
            .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut count = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if board[i][j] == '*' {
                    continue;
                }

                let mut num = 0;
                for (dx, dy) in dir {
                    let (nx, ny) = (i as i64 + dx, j as i64 + dy);

                    if nx >= 0
                        && nx < n as i64
                        && ny >= 0
                        && ny < n as i64
                        && board[nx as usize][ny as usize] == '*'
                    {
                        num += 1;
                    }
                }

                count[i][j] = num;
            }
        }

        let mut answer = 0;
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; n]; n];

        for i in 0..n {
            for j in 0..n {
                if visited[i][j] || board[i][j] == '*' || count[i][j] != 0 {
                    continue;
                }

                answer += 1;
                visited[i][j] = true;
                queue.push_back((i, j));

                while let Some((x, y)) = queue.pop_front() {
                    for (dx, dy) in dir {
                        let (nx, ny) = (x as i64 + dx, y as i64 + dy);

                        if nx >= 0 && nx < n as i64 && ny >= 0 && ny < n as i64 {
                            let (nx, ny) = (nx as usize, ny as usize);

                            if !visited[nx][ny] && board[nx][ny] == '.' {
                                visited[nx][ny] = true;
                                if count[nx][ny] == 0 {
                                    queue.push_back((nx, ny));
                                }
                            }
                        }
                    }
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                if board[i][j] == '.' && !visited[i][j] {
                    answer += 1;
                }
            }
        }
        writeln!(out, "Case #{}: {}", t, answer).unwrap();
    }
}
