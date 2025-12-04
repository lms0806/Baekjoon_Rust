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

    let (n, b) = (scan.token::<usize>(), scan.token::<usize>());
    let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    let mut dp = vec![vec![i64::MIN; 2]; b + 1];
    dp[0][0] = 0;
    dp[1][1] = 0;

    for i in 1..n {
        for j in (1..=b).rev() {
            dp[j][0] = dp[j][0].max(dp[j][1]);
            dp[j][1] = dp[j - 1][0].max(dp[j - 1][1] + arr[i]);
        }
    }

    write!(out, "{}", dp[b][0].max(dp[b][1])).unwrap();
}
