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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (mut arr_h, mut arr_l) = (
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
        (0..m).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
    );

    arr_h.sort_unstable();
    arr_l.sort_unstable();

    let mut answer = 0;
    let (mut idx_h, mut idx_l) = (0, 0);
    while idx_h < n && idx_l < m {
        if arr_h[idx_h] > arr_l[idx_l] {
            answer += 1;
            idx_h += 1;
            idx_l += 1;
        } else {
            idx_h += 1;
        }
    }
    write!(out, "{}", answer + n).unwrap();
}
