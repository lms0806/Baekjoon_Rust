use std::collections::BTreeSet;
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

    let alpha = scan.token::<String>().chars().collect::<Vec<_>>();
    let origin = scan.token::<String>();
    let arr = origin.chars().collect::<Vec<_>>();

    let mut answer = BTreeSet::new();
    for i in 0..arr.len() {
        let mut temp = String::new();
        for j in 0..arr.len() {
            if i != j {
                temp.push(arr[j]);
            }
        }
        answer.insert(temp);
    }

    for i in 0..arr.len() {
        for &c in &alpha {
            let mut temp = String::new();

            for j in 0..arr.len() {
                temp.push(if i == j { c } else { arr[j] });
            }

            answer.insert(temp);
        }
    }

    for i in 0..=arr.len() {
        for &c in &alpha {
            let mut temp = String::new();

            for j in 0..i {
                temp.push(arr[j]);
            }
            temp.push(c);
            for j in i..arr.len() {
                temp.push(arr[j]);
            }

            answer.insert(temp);
        }
    }

    answer.remove(&origin);

    for word in answer {
        writeln!(out, "{}", word).unwrap();
    }
}
