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

    for _ in 0..scan.token::<usize>() {
        let n = scan.token::<usize>();
        let mut arr = (0..n)
            .map(|_| (scan.token::<i64>(), scan.token::<i64>()))
            .collect::<Vec<_>>();

        arr.sort_unstable();

        let (sum_x, sum_y) = (arr[0].0 + arr[n - 1].0, arr[0].1 + arr[n - 1].1);

        let mut check = true;
        for i in 0..=(n / 2) {
            let (l, r) = (arr[i], arr[n - i - 1]);

            if l.0 + r.0 != sum_x || l.1 + r.1 != sum_y {
                check = false;
                break;
            }
        }

        writeln!(out, "{}", if check { "yes" } else { "no" }).unwrap();
    }
}
