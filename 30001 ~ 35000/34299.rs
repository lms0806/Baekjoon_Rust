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

    let (s, e) = (
        change(scan.token::<String>()),
        change(scan.token::<String>()),
    );

    write!(
        out,
        "{} {} {}",
        e / (12 * 3600) - s / (12 * 3600),
        e / 3600 - s / 3600,
        e / 60 - s / 60
    )
    .unwrap();
}

fn change(str: String) -> i64 {
    let num = str
        .split(":")
        .map(|str| str.parse().unwrap())
        .collect::<Vec<i64>>();

    num[0] * 3600 + num[1] * 60 + num[2]
}
