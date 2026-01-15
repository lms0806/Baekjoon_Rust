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

    let mut alpha = [0; 26];
    for ch in scan.token::<String>().bytes() {
        alpha[(ch - b'a') as usize] += 1;
    }

    let mut odd = vec![];
    for i in 0..alpha.len() {
        if alpha[i] % 2 == 1 {
            odd.push(i);
        }
    }

    let mut answer = String::new();
    for i in 0..26 {
        if alpha[i] == 0 {
            continue;
        }

        for _ in 0..alpha[i] / 2 {
            answer.push((b'a' + i as u8) as char);
        }
    }

    if odd.is_empty() {
        writeln!(out, "1").unwrap();
        write!(
            out,
            "{}{}",
            answer,
            answer.chars().rev().collect::<String>()
        )
        .unwrap();
    } else {
        writeln!(out, "{}", odd.len()).unwrap();
        writeln!(
            out,
            "{}{}{}",
            answer,
            (b'a' + odd[0] as u8) as char,
            answer.chars().rev().collect::<String>()
        )
        .unwrap();
        for i in 1..odd.len() {
            writeln!(out, "{}", (b'a' + odd[i] as u8) as char).unwrap();
        }
    }
}
