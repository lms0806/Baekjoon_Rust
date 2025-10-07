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

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<i64>());

        if n == 0 && m == 0 {
            break;
        }

        let mut check = false;
        let (mut min, mut ticket, mut price) = (f64::MAX, 0, 0);
        for _ in 0..n {
            let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

            if a > m {
                continue;
            }

            check = true;
            let num = b as f64 / a as f64;
            if num < min {
                min = num;
                ticket = a;
                price = b;
            } else if num == min {
                if a > ticket {
                    ticket = a;
                    price = b;
                }
            }
        }

        if check {
            writeln!(out, "Buy {} tickets for ${}", ticket, price).unwrap();
        } else {
            writeln!(out, "No suitable tickets offered").unwrap();
        }
    }
}
