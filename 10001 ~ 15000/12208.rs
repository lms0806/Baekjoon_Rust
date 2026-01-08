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
        let (n, op) = (scan.token::<usize>(), scan.token::<String>());

        let mut arr = (0..n)
            .map(|_| (0..n).map(|_| scan.token::<usize>()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        match op.as_str() {
            "left" => {
                for i in 0..n {
                    arr[i] = change(n, &arr[i]);
                }
            }
            "right" => {
                for i in 0..n {
                    let mut row = arr[i].clone();
                    row.reverse();
                    row = change(n, &row);
                    row.reverse();
                    arr[i] = row;
                }
            }
            "up" => {
                for j in 0..n {
                    let mut col = (0..n).map(|i| arr[i][j]).collect::<Vec<_>>();
                    col = change(n, &col);
                    for i in 0..n {
                        arr[i][j] = col[i];
                    }
                }
            }
            "down" => {
                for j in 0..n {
                    let mut col = (0..n).map(|i| arr[i][j]).collect::<Vec<_>>();
                    col.reverse();
                    col = change(n, &col);
                    col.reverse();
                    for i in 0..n {
                        arr[i][j] = col[i];
                    }
                }
            }
            _ => {}
        }

        writeln!(out, "Case #{}: ", i).unwrap();
        for i in 0..n {
            for j in 0..n {
                write!(out, "{} ", arr[i][j]).unwrap();
            }
            writeln!(out).unwrap();
        }
    }
}

fn change(n: usize, line: &[usize]) -> Vec<usize> {
    let v = line.iter().cloned().filter(|&x| x != 0).collect::<Vec<_>>();
    let mut result = vec![];
    let mut idx = 0;
    while idx < v.len() {
        if idx + 1 < v.len() && v[idx] == v[idx + 1] {
            result.push(v[idx] * 2);
            idx += 2;
        } else {
            result.push(v[idx]);
            idx += 1;
        }
    }
    result.resize(n, 0);
    result
}
