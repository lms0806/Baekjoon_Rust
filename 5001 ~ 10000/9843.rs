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
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let arr = (0..n)
        .map(|_| {
            scan.line()
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut idx = 0;
    let mut register = -1;
    let mut stack = vec![];

    while idx < n {
        match arr[idx][0].as_str() {
            "PUSH" => {
                stack.push(arr[idx][1].parse::<i64>().unwrap());
            }
            "STORE" => {
                register = stack.pop().unwrap();
            }
            "LOAD" => {
                stack.push(register);
            }
            "PLUS" => {
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a + b);
            }
            "TIMES" => {
                let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a * b);
            }
            "IFZERO" => {
                if stack.pop().unwrap() == 0 {
                    idx = arr[idx][1].parse::<usize>().unwrap() - 1;
                }
            }
            "DONE" => {
                write!(out, "{}", stack.pop().unwrap()).unwrap();
                return;
            }
            _ => {}
        }
        idx += 1;
    }
}
