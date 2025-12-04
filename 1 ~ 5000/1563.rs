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
    let mods = 1000000;
    let mut dp = vec![vec![vec![0; 3]; 2]; n + 1];

    dp[1][0][0] = 1;
    dp[1][0][1] = 1;
    dp[1][1][0] = 1;

    for i in 2..=n {
        dp[i][0][0] = (dp[i - 1][0][0] + dp[i - 1][0][1] + dp[i - 1][0][2]) % mods;
        dp[i][0][1] = dp[i - 1][0][0] % mods;
        dp[i][0][2] = dp[i - 1][0][1] % mods;
        dp[i][1][0] = (dp[i - 1][0][0]
            + dp[i - 1][0][1]
            + dp[i - 1][0][2]
            + dp[i - 1][1][0]
            + dp[i - 1][1][1]
            + dp[i - 1][1][2])
            % mods;
        dp[i][1][1] = dp[i - 1][1][0] % mods;
        dp[i][1][2] = dp[i - 1][1][1] % mods;
    }

    write!(
        out,
        "{}",
        (dp[n][0][0] + dp[n][0][1] + dp[n][0][2] + dp[n][1][0] + dp[n][1][1] + dp[n][1][2]) % mods
    )
    .unwrap();
}
