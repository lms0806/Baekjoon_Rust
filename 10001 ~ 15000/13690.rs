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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (v, n, m) = (
            scan.token::<f64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        if v == 0.0 && n + m == 0 {
            return;
        }

        let (n2, m2) = (n % 100, m % 100);

        let answer = if n % 10000 == m % 10000 {
            v * 3000.0
        } else if n % 1000 == m % 1000 {
            v * 500.0
        } else if n2 == m2 {
            v * 50.0
        } else {
            let g1 = if n2 == 0 { 25 } else { (n2 - 1) / 4 + 1 };
            let g2 = if m2 == 0 { 25 } else { (m2 - 1) / 4 + 1 };

            if g1 == g2 {
                v * 16.0
            } else {
                0.0
            }
        };
        writeln!(out, "{:.2}", answer).unwrap();
    }
}
