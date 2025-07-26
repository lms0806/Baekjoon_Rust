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

    let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
    let (mut dp, mut back_dp) = (vec![1; n], vec![1; n]);

    for i in 0..n {
        for j in 0..i {
            if arr[j] < arr[i] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
            }
        }
    }

    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if arr[i] > arr[j] && back_dp[j] + 1 > back_dp[i] {
                back_dp[i] = back_dp[j] + 1;
            }
        }
    }

    write!(
        out,
        "{}",
        dp.iter()
            .zip(back_dp.iter())
            .map(|(&a, &b)| a + b - 1)
            .max()
            .unwrap_or(1)
    )
    .unwrap();
}
