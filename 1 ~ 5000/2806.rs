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

    let n = scan.token::<usize>();

    let mut dp = vec![vec![0; 2]; n];
    let arr = scan.token::<String>().trim().chars().collect::<Vec<_>>();
    dp[0][0] = if arr[0] == 'A' { 0 } else { 1 };
    dp[0][1] = 1 - dp[0][0];

    for i in 1..n {
        let (idx1, idx2) = if arr[i] == 'A' { (0, 1) } else { (1, 0) };

        dp[i][idx1] = dp[i - 1][idx1].min(dp[i - 1][idx2] + 1);
        dp[i][idx2] = dp[i - 1][idx1].min(dp[i - 1][idx2]) + 1;
    }

    write!(out, "{}", dp[n - 1][0].min(dp[n - 1][1] + 1)).unwrap();
}
