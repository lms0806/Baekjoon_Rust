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

    while let Some(x1) = scan.token_eof::<f64>() {
        let y1 = scan.token::<f64>();
        let (x2, y2) = (scan.token::<f64>(), scan.token::<f64>());
        let (x3, y3) = (scan.token::<f64>(), scan.token::<f64>());
        let (x4, y4) = (scan.token::<f64>(), scan.token::<f64>());

        let (rx, ry) = if same(x1, y1, x3, y3) {
            (x2 + x4 - x1, y2 + y4 - y1)
        } else if same(x1, y1, x4, y4) {
            (x2 + x3 - x1, y2 + y3 - y1)
        } else if same(x2, y2, x3, y3) {
            (x1 + x4 - x2, y1 + y4 - y2)
        } else {
            (x1 + x3 - x2, y1 + y3 - y2)
        };
        writeln!(out, "{:.3} {:.3}", rx, ry).unwrap();
    }
}

fn same(x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
    (x1 - x2).abs() < 1e-9 && (y1 - y2).abs() < 1e-9
}
