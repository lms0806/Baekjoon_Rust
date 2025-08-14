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

    let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

    let (mut ans, total) = (0, 9 * 17);
    if a == b {
        ans = total - (10 - a)
    } else {
        let num = (a + b) % 10;
        for i in 1..11 {
            for j in i + 1..11 {
                if num > (i + j) % 10 {
                    if i == a && j == b {
                        continue;
                    }

                    if i == a || j == a || i == b || j == b {
                        ans += 2;
                    } else {
                        ans += 4;
                    }
                }
            }
        }
    }

    write!(out, "{:.3}", ans as f64 / total as f64).unwrap();
}
