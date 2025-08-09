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

    let n = scan.token::<usize>();
    let (a, b, c) = (
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
    );

    let mut num = -1;
    for i in 0..n {
        let (min, max) = (a[i].min(b[i].min(c[i])), a[i].max(b[i].max(c[i])));

        if num >= max {
            write!(out, "NO").unwrap();
            return;
        }

        if min <= num && num < max {
            num += 1;
        } else {
            num = min;
        }
    }
    write!(out, "YES").unwrap();
}
