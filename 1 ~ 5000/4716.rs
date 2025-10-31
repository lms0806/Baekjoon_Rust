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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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

    loop {
        let (n, mut a, mut b) = (
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if n == 0 && a + b == 0 {
            break;
        }

        let mut arr = vec![];
        for _ in 0..n {
            arr.push((
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            ));
        }

        arr.sort_unstable_by(|a, b| (b.1 - b.2).abs().cmp(&(a.1 - a.2).abs()));

        let mut answer = 0;
        for (k, da, db) in arr {
            let (fa, fb) = (
                k.min(a) * da + (k - k.min(a)).max(0) * db,
                k.min(b) * db + (k - k.min(b)).max(0) * da,
            );

            if fa <= fb {
                let mut num = k.min(a);
                answer += num * da;
                a -= num;

                num = k - num;
                answer += num * db;
                b -= num;
            } else {
                let mut num = k.min(b);
                answer += num * db;
                b -= num;

                num = k - num;
                answer += num * da;
                a -= num;
            }
        }
        writeln!(out, "{}", answer).unwrap();
    }
}
