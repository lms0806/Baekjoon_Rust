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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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

    let n = scan.token::<usize>();
    let arr = (0..n).map(|_| scan.token::<usize>()).collect::<Vec<_>>();
    let price = scan.token::<usize>();
    let mut dp = vec!["0".to_string(); price + 1];

    for i in 0..n {
        if arr[i] > price {
            continue;
        }
        dp[arr[i]] = i.to_string();
    }

    for i in 0..=price {
        if dp[i] == "0" {
            continue;
        }

        for j in 0..n {
            if i + arr[j] <= price {
                let new = dp[i].clone() + j.to_string().as_str();
                let num = &dp[i + arr[j]];

                if check(&new, &num) {
                    dp[i + arr[j]] = new;
                }
            }
        }
    }

    let mut answer = "0".to_string();
    for v in dp {
        if check(&v, &answer) {
            answer = v.clone();
        }
    }
    write!(out, "{}", answer).unwrap();
}

fn check(a: &String, b: &String) -> bool {
    if a.len() != b.len() {
        return a.len() > b.len();
    }
    a > b
}
