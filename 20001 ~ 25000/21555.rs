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

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let (a, b) = (
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
    );
    let mut dp = vec![vec![0; 2]; n];

    dp[0][0] = a[0];
    dp[0][1] = b[0];
    for i in 1..n {
        dp[i][0] = (dp[i - 1][0] + a[i]).min(dp[i - 1][1] + k + a[i]);
        dp[i][1] = (dp[i - 1][1] + b[i]).min(dp[i - 1][0] + k + b[i]);
    }

    write!(out, "{}", dp[n - 1][0].min(dp[n - 1][1])).unwrap();
}
