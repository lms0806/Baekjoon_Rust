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

    let ((h1, m1), (h2, m2)) = (
        parse(&scan.token::<String>()),
        parse(&scan.token::<String>()),
    );
    let (sh, th, sm, tm) = (h1 * 60 + m1, h2 * 60 + m2, m1 * 12, m2 * 12);

    let (h, m) = (change(th - sh), change(tm - sm));

    let mut answer = i64::MAX;
    for i in -20..=20 {
        let num = m + 720 * i;
        let hour = change(num / 12);

        if hour == h {
            answer = answer.min(num.abs());
        }
    }
    write!(out, "{}", answer / 2).unwrap();
}

fn change(x: i64) -> i64 {
    ((x % 720) + 720) % 720
}

fn parse(s: &str) -> (i64, i64) {
    let parts: Vec<_> = s.split(':').collect();
    let (h, m) = (
        parts[0].parse::<i64>().unwrap() % 12,
        parts[1].parse::<i64>().unwrap(),
    );

    (h, m)
}
