use io::Write;
use std::collections::HashSet;
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

    let (d, p) = (scan.token::<i64>(), scan.token::<i64>());

    if 2i64.pow(p as u32) > 10i64.pow(d as u32) - 1 {
        write!(out, "-1").unwrap();
    } else {
        let mut answer = 0;
        dfs(10i64.pow(d as u32), p, 1, &mut answer, 1);
        write!(out, "{}", answer).unwrap();
    }
}

fn dfs(d: i64, p: i64, num: i64, answer: &mut i64, prev: i64) {
    if p == 0 {
        *answer = (*answer).max(num);
        return;
    }

    for i in prev.max(2)..=9 {
        if num * i < d {
            dfs(d, p - 1, num * i, answer, i);
        }
    }
}
