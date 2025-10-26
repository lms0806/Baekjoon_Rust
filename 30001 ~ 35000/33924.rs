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

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).expect("Failed read");

        if bytes_read == 0 {
            None
        } else {
            Some(input)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut n, mut m) = (scan.token::<i64>(), scan.token::<i64>());
    let _ = scan.token::<usize>();

    for ch in scan.line().trim().chars() {
        if ch == 'A' {
            n += 2;
        } else if ch == 'B' {
            if n % 2 == 1 {
                n += 1;
            } else {
                n -= 1;
            }

            m += 1;
        } else if ch == 'C' {
            if n == 1 {
                n = 4;
            } else if n == 2 {
                n = 3;
            } else if n == 3 {
                n = 2;
            } else {
                n = 1;
            }

            m += 1;
        } else {
            if m == 1 {
                if n == 1 {
                    m += 1;
                } else {
                    n -= 1;
                }
            } else {
                if n == 4 {
                    m -= 1;
                } else {
                    n += 1;
                }
            }
        }

        if n > 4 {
            n -= 4;
        }

        if m > 2 {
            m -= 2;
        }
    }
    write!(out, "{} {}", n, m).unwrap();
}
