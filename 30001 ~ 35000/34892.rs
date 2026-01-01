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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let (a, b, c, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (e, f, g, h) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut count = 0;
    let mut answer = (0, 0, 0);

    for x in 0..=n {
        for y in 0..=(n - x) {
            let z = n - x - y;

            if a * x + b * y + c * z == d && e * x + f * y + g * z == h {
                count += 1;

                if count == 1 {
                    answer = (x, y, z);
                } else {
                    write!(out, "1").unwrap();
                    return;
                }
            }
        }
    }

    if count == 0 {
        write!(out, "2").unwrap();
    } else {
        writeln!(out, "0").unwrap();
        write!(out, "{} {} {}", answer.0, answer.1, answer.2).unwrap();
    }
}
