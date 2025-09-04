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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let arr = (0..n)
        .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut answer = 0;
    let dirs: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..n {
        for j in 0..m {
            if arr[i][j] != 'F' {
                continue;
            }

            for &(dx, dy) in &dirs {
                let (nx1, ny1, nx2, ny2) = (
                    i as i64 + dx,
                    j as i64 + dy,
                    i as i64 + 2 * dx,
                    j as i64 + 2 * dy,
                );

                if nx1 < 0 || ny1 < 0 || nx2 < 0 || ny2 < 0 || nx2 >= n as i64 || ny2 >= m as i64 {
                    continue;
                }

                if arr[nx1 as usize][ny1 as usize] == 'O' && arr[nx2 as usize][ny2 as usize] == 'X'
                {
                    answer += 1;
                }
            }
        }
    }

    write!(out, "{}", answer).unwrap();
}
