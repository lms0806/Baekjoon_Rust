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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).expect("Failed read");

        if bytes_read == 0 {
            None
        } else {
            Some(input)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut dp = vec![0; n + 1];
    let mut arr = vec![vec![]; n + 1];

    for _ in 1..n {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        arr[u].push(v);
        arr[v].push(u);
    }

    let mut answer = 0;
    dfs(n, &mut dp, &arr, 1, 0, &mut answer);

    write!(out, "{}", answer).unwrap();
}

fn dfs(
    n: usize,
    dp: &mut Vec<usize>,
    arr: &Vec<Vec<usize>>,
    idx: usize,
    parent: usize,
    max: &mut usize,
) -> usize {
    if dp[idx] != 0 {
        return dp[idx];
    }

    dp[idx] = 1;

    for &next in &arr[idx] {
        if next == parent {
            continue;
        }

        let num = dfs(n, dp, arr, next, idx, max);
        *max = (*max).max(num * (n - num));
        dp[idx] += num;
    }

    dp[idx]
}
