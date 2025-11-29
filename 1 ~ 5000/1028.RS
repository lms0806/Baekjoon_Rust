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
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    if n == 1 && m == 1 {
        write!(out, "{}", scan.token::<i64>()).unwrap();
        return;
    }

    let arr = (0..n)
        .map(|_| {
            scan.line()
                .trim()
                .chars()
                .map(|c| (c as u8 - b'0') as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut dp = vec![vec![vec![0; 4]; m]; n];

    for i in (0..n).rev() {
        for j in 0..m {
            if arr[i][j] == 0 {
                continue;
            }

            dp[i][j][0] = 1 + if i + 1 < n && j > 0 {
                dp[i + 1][j - 1][0]
            } else {
                0
            };
        }
    }

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if arr[i][j] == 0 {
                continue;
            }

            dp[i][j][1] = 1 + if i + 1 < n && j + 1 < m {
                dp[i + 1][j + 1][1]
            } else {
                0
            };
        }
    }

    for i in 0..n {
        for j in 0..m {
            if arr[i][j] == 0 {
                continue;
            }

            dp[i][j][2] = 1 + if i > 0 && j > 0 {
                dp[i - 1][j - 1][2]
            } else {
                0
            };
        }
    }

    for i in 0..n {
        for j in (0..m).rev() {
            if arr[i][j] == 0 {
                continue;
            }

            dp[i][j][3] = 1 + if i > 0 && j + 1 < m {
                dp[i - 1][j + 1][3]
            } else {
                0
            };
        }
    }

    let mut answer = 0;
    for i in 0..n {
        for j in 0..m {
            let len = dp[i][j][2].min(dp[i][j][3]);
            for k in (0..len).rev() {
                if i < 2 * k {
                    continue;
                }

                if dp[i - 2 * k][j][0] > k && dp[i - 2 * k][j][1] > k {
                    answer = answer.max(k + 1);
                    break;
                }
            }
        }
    }

    write!(out, "{}", answer).unwrap();
}
