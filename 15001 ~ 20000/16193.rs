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

    let n = scan.token::<usize>();
    let mut arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    arr.sort_unstable();

    let mut answer = 0;
    let (mut l, mut r) = (1, n - 1);
    let (mut front, mut back) = (arr[0], arr[0]);
    while l <= r {
        let (v1, v2, v3, v4) = (
            arr[l].abs_diff(front),
            arr[r].abs_diff(front),
            arr[l].abs_diff(back),
            arr[r].abs_diff(back),
        );

        let max = v1.max(v2.max(v3.max(v4)));

        answer += max;

        if max == v1 {
            front = arr[l];
            l += 1;
        } else if max == v2 {
            front = arr[r];
            r -= 1;
        } else if max == v3 {
            back = arr[l];
            l += 1;
        } else {
            back = arr[r];
            r -= 1;
        }
    }
    write!(out, "{}", answer).unwrap();
}
