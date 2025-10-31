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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    while let Some(mut x) = scan.token_eof::<i64>() {
        x *= 10000000;

        let n = scan.token::<usize>();
        let mut arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

        if n < 2 {
            writeln!(out, "danger").unwrap();
            continue;
        }

        arr.sort_unstable();

        let mut check = false;
        let (mut s, mut e) = (0, n - 1);
        while s < e {
            if arr[s] + arr[e] > x {
                e -= 1;
            } else if arr[s] + arr[e] < x {
                s += 1;
            } else {
                writeln!(out, "yes {} {}", arr[s], arr[e]).unwrap();
                check = true;
                break;
            }
        }

        if !check {
            writeln!(out, "danger").unwrap();
        }
    }
}
