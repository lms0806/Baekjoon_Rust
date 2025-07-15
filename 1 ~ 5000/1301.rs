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

    let mut arr = vec![0; 5];
    for i in 0..n {
        arr[i] = scan.token::<i64>();
    }

    write!(
        out,
        "{}",
        dfs(
            5,
            5,
            arr[0],
            arr[1],
            arr[2],
            arr[3],
            arr[4],
            &mut [[[[[[[-1i64; 11]; 11]; 11]; 11]; 11]; 6]; 6]
        )
    )
    .unwrap();
}

fn dfs(
    cur: usize,
    prev: usize,
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    dp: &mut [[[[[[[i64; 11]; 11]; 11]; 11]; 11]; 6]; 6],
) -> i64 {
    if a + b + c + d + e == 0 {
        return 1;
    }

    let mut sum = dp[cur][prev][a as usize][b as usize][c as usize][d as usize][e as usize];

    if sum != -1 {
        return sum;
    }

    sum = 0;
    if a > 0 && prev != 0 && cur != 0 {
        sum += dfs(0, cur, a - 1, b, c, d, e, dp);
    }
    if b > 0 && prev != 1 && cur != 1 {
        sum += dfs(1, cur, a, b - 1, c, d, e, dp);
    }
    if c > 0 && prev != 2 && cur != 2 {
        sum += dfs(2, cur, a, b, c - 1, d, e, dp);
    }
    if d > 0 && prev != 3 && cur != 3 {
        sum += dfs(3, cur, a, b, c, d - 1, e, dp);
    }
    if e > 0 && prev != 4 && cur != 4 {
        sum += dfs(4, cur, a, b, c, d, e - 1, dp);
    }

    dp[cur][prev][a as usize][b as usize][c as usize][d as usize][e as usize] = sum;
    sum
}
