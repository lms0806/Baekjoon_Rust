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

    for i in 1..=scan.token::<usize>() {
        let n = scan.token::<usize>();

        let arr = (0..n * n)
            .map(|_| {
                (0..n * n)
                    .map(|_| scan.token::<usize>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        writeln!(
            out,
            "Case #{}: {}",
            i,
            if check(n, &arr) { "Yes" } else { "No" }
        )
        .unwrap();
    }
}

fn check(n: usize, arr: &Vec<Vec<usize>>) -> bool {
    for i in 0..arr.len() {
        let (mut col, mut row) = (vec![false; arr.len() + 1], vec![false; arr.len() + 1]);

        for j in 0..arr[i].len() {
            if arr[i][j] > arr.len() || arr[j][i] > arr.len() {
                return false;
            }

            if col[arr[i][j]] || row[arr[j][i]] {
                return false;
            }

            col[arr[i][j]] = true;
            row[arr[j][i]] = true;
        }
    }

    for row in 0..n {
        for col in 0..n {
            let mut block = vec![false; arr.len() + 1];

            for i in 0..n {
                for j in 0..n {
                    let val = arr[row * n + i][col * n + j];
                    if val == 0 || val > arr.len() || block[val] {
                        return false;
                    }
                    block[val] = true;
                }
            }
        }
    }
    true
}
