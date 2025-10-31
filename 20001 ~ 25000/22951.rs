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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let arr = (0..n * k)
        .map(|_| scan.token::<usize>())
        .collect::<Vec<_>>();
    let mut visited = vec![false; n * k];

    let (mut count, mut idx) = (0, 0);
    while count != n * k - 1 {
        let mut moved = arr[idx];

        count += 1;

        visited[idx] = true;

        let mut i = 1;
        while i <= moved {
            if visited[(idx + i) % (n * k)] {
                moved += 1;
            }
            i += 1;
        }

        idx = (idx + moved) % (n * k);
    }
    writeln!(out, "{} {}", idx / k + 1, arr[idx]).unwrap();
}
