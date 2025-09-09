use io::Write;
use std::f64::consts::PI;
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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            let bytes_read = self
                .reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");

            if bytes_read == 0 {
                return None;
            }

            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (x1, y1, x2, y2, x3, y3) = match (
            scan.token_eof::<f64>(),
            scan.token_eof::<f64>(),
            scan.token_eof::<f64>(),
            scan.token_eof::<f64>(),
            scan.token_eof::<f64>(),
            scan.token_eof::<f64>(),
        ) {
            (Some(x1), Some(y1), Some(x2), Some(y2), Some(x3), Some(y3)) => {
                (x1, y1, x2, y2, x3, y3)
            }
            _ => break, // EOF
        };

        let (a, b, c) = (
            dist(x1, y1, x2, y2),
            dist(x1, y1, x3, y3),
            dist(x2, y2, x3, y3),
        );

        let cos_a = (b * b + c * c - a * a) / (2.0 * b * c);
        let sin_a = (1.0 - cos_a * cos_a).sqrt();
        let r = a / (2.0 * sin_a);

        writeln!(out, "{:.2}", 2.0 * PI * r).unwrap();
    }
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
}
