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

    let mut num = scan.token::<i64>();
    let (mut a, mut b, mut c, mut d, mut e) = (0, 0, 0, 0, 0);

    if num / 150 != 0 {
        e = num / 150;
        num %= 150;
    }
    if num / 30 != 0 {
        d = num / 30;
        num %= 30;
    }
    if num / 15 != 0 {
        c = num / 15;
        num %= 15;
    }
    if num / 5 != 0 {
        b = num / 5;
        num %= 5;
    }
    if num / 1 != 0 {
        a = num;
    }
    write!(out, "{} {} {} {} {}", a, b, c, d, e).unwrap();
}
