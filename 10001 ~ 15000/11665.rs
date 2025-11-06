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

    let (mut max_x1, mut max_y1, mut max_z1) = (i64::MIN, i64::MIN, i64::MIN);
    let (mut min_x2, mut min_y2, mut min_z2) = (i64::MAX, i64::MAX, i64::MAX);
    for _ in 0..scan.token::<usize>() {
        max_x1 = max_x1.max(scan.token::<i64>());
        max_y1 = max_y1.max(scan.token::<i64>());
        max_z1 = max_z1.max(scan.token::<i64>());

        min_x2 = min_x2.min(scan.token::<i64>());
        min_y2 = min_y2.min(scan.token::<i64>());
        min_z2 = min_z2.min(scan.token::<i64>());
    }
    write!(
        out,
        "{}",
        (min_x2 - max_x1).max(0) * (min_y2 - max_y1).max(0) * (min_z2 - max_z1).max(0)
    )
    .unwrap();
}
