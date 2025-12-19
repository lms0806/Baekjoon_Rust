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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut count = vec![0; 64];
    for _ in 0..n {
        let (subject, fruit, color) = change(
            &scan.token::<String>(),
            &scan.token::<String>(),
            &scan.token::<String>(),
        );

        for s in [subject, 3] {
            for f in [fruit, 3] {
                for c in [color, 3] {
                    count[s * 16 + f * 4 + c] += 1;
                }
            }
        }
    }

    for _ in 0..m {
        let (subject, fruit, color) = change(
            &scan.token::<String>(),
            &scan.token::<String>(),
            &scan.token::<String>(),
        );

        writeln!(out, "{}", count[subject * 16 + fruit * 4 + color]).unwrap();
    }
}

fn change(a: &str, b: &str, c: &str) -> (usize, usize, usize) {
    let subject = match a {
        "kor" => 0,
        "eng" => 1,
        "math" => 2,
        _ => 3,
    };
    let fruit = match b {
        "apple" => 0,
        "pear" => 1,
        "orange" => 2,
        _ => 3,
    };
    let color = match c {
        "red" => 0,
        "blue" => 1,
        "green" => 2,
        _ => 3,
    };
    (subject, fruit, color)
}
