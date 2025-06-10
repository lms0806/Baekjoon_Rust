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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let a = scan.token::<String>().chars().collect::<Vec<_>>();

    let mut answer = 0;
    for i in 0..a.len() {
        if a[i] < 'D' {
            answer += 3;
        } else if a[i] < 'G' {
            answer += 4;
        } else if a[i] < 'J' {
            answer += 5;
        } else if a[i] < 'M' {
            answer += 6;
        } else if a[i] < 'P' {
            answer += 7;
        } else if a[i] < 'T' {
            answer += 8;
        } else if a[i] < 'W' {
            answer += 9;
        } else {
            answer += 10;
        }
    }
    write!(out, "{}", answer);
}
