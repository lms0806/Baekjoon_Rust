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
            buf_str: Vec::with_capacity(1 << 16),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    #[inline(always)]
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
            };
        }
    }

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok()?);
            }
            self.buf_str.clear();
            let n = self.reader.read_until(b'\n', &mut self.buf_str).ok()?;
            if n == 0 {
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_iter = "".split_ascii_whitespace();
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    if k == 1 {
        write!(out, "0").unwrap();
        return;
    }

    let arr = (0..n)
        .map(|_| (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut answer = i64::MIN;
    let mut num = vec![];

    dfs(0, n, k, 0, &mut num, &arr, &mut answer);

    write!(out, "{}", answer).unwrap();
}

fn dfs(
    idx: usize,
    n: usize,
    k: usize,
    count: usize,
    num: &mut Vec<usize>,
    arr: &Vec<Vec<i64>>,
    answer: &mut i64,
) {
    if count == k {
        let mut sum = 0;
        for i in 0..num.len() {
            for j in i + 1..num.len() {
                sum += arr[num[i]][num[j]];
            }
        }
        *answer = (*answer).max(sum);
        return;
    }

    if idx == n {
        return;
    }

    if count + n - idx < k {
        return;
    }

    dfs(idx + 1, n, k, count, num, arr, answer);
    num.push(idx);
    dfs(idx + 1, n, k, count + 1, num, arr, answer);
    num.pop();
}
