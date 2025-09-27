use io::Write;
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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let arr = (0..n)
        .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dr = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut answer = 0;
    for i in 0..n {
        for j in 0..m {
            if arr[i][j] != 'X' {
                continue;
            }

            for (dx, dy) in dr {
                let (nx, ny) = (i as i64 + dx, j as i64 + dy);
                if nx < 0 || nx >= n as i64 || ny < 0 || ny >= m as i64 {
                    continue;
                }

                if arr[nx as usize][ny as usize] == 'Y' {
                    answer += 1;
                }
            }
        }
    }
    write!(out, "{}", answer).unwrap();
}
