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

    for _ in 0..scan.token::<usize>() {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        let arr = (0..n)
            .map(|_| (0..m).map(|_| scan.token::<char>()).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
        if let Some(answer) = solve(arr) {
            writeln!(
                out,
                "{} {} {} {} {} {}",
                answer.0, answer.1, answer.2, answer.3, answer.4, answer.5
            )
            .unwrap();
        } else {
            writeln!(out, "no set found").unwrap();
        }
    }
}

fn solve(arr: Vec<Vec<char>>) -> Option<(usize, usize, usize, usize, usize, usize)> {
    for i in 0..arr.len() {
        for j in 2..arr[i].len() {
            if arr[i][j - 2] == arr[i][j - 1] && arr[i][j - 1] == arr[i][j] {
                return Some((i + 1, j - 1, i + 1, j, i + 1, j + 1));
            }
        }
    }

    for i in 0..arr[0].len() {
        for j in 2..arr.len() {
            if arr[j - 2][i] == arr[j - 1][i] && arr[j - 1][i] == arr[j][i] {
                return Some((j - 1, i + 1, j, i + 1, j + 1, i + 1));
            }
        }
    }
    None
}
