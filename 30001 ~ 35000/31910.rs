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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.trim().to_string()
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).ok()?;
        if bytes_read == 0 {
            return None; // EOF
        }
        Some(input.trim().to_string())
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let arr = (0..n)
        .map(|_| (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut dp = vec![vec![0; n]; n];

    dp[0][0] = arr[0][0];
    for i in 1..n {
        dp[i][0] = (dp[i - 1][0] << 1) + arr[i][0];
        dp[0][i] = (dp[0][i - 1] << 1) + arr[0][i];
    }

    for i in 1..n {
        for j in 1..n {
            dp[i][j] = (dp[i - 1][j].max(dp[i][j - 1]) << 1) + arr[i][j];
        }
    }

    write!(out, "{}", dp[n - 1][n - 1]).unwrap();
}
