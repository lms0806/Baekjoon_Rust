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

    let mut dp = vec![vec![vec![0i128; 2]; 101]; 101];
    dp[1][0][0] = 1;
    dp[1][0][1] = 1;

    for k in 0..101 {
        for n in 2..101 {
            dp[n][k][1] += dp[n - 1][k][0];

            if k > 0 {
                dp[n][k][1] += dp[n - 1][k - 1][1];
            }

            dp[n][k][0] += dp[n - 1][k][0] + dp[n - 1][k][1];
        }
    }

    for _ in 0..scan.token::<usize>() {
        let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
        writeln!(out, "{}", dp[n][k][0] + dp[n][k][1]).unwrap();
    }
}
