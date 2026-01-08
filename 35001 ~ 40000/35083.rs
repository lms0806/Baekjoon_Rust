use std::cmp::Ordering::{Equal, Greater};
use std::collections::HashMap;
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
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let mut map = HashMap::new();
    for _ in 0..scan.token::<usize>() {
        for i in 1..=10 {
            let entry = map.entry(scan.token::<String>()).or_insert([0; 11]);

            entry[0] += 1;
            entry[i] += 1;
        }
    }

    let mut best_rank = [0; 11];
    let mut answer = String::new();
    let (mut first, mut tie) = (true, true);
    for (name, rank) in map {
        if first {
            best_rank = rank;
            answer = name;
            first = false;
            tie = false;
            continue;
        }

        match rank.cmp(&best_rank) {
            Greater => {
                best_rank = rank;
                answer = name;
                tie = false;
            }
            Equal => {
                tie = true;
            }
            _ => {}
        }
    }

    write!(out, "{}", if tie { "tie" } else { &answer }).unwrap();
}
