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

    let (s, c) = (scan.token::<String>(), scan.token::<i64>());

    let mut str: Vec<(char, i64)> = Vec::new();
    let mut now = ' ';
    let mut num = 0;

    for ch in s.chars() {
        if ch.is_ascii_alphabetic() {
            if num > 0 {
                str.push((now, num));
                num = 0;
            }
            now = ch;
        } else {
            num = num * 10 + (ch as i64 - '0' as i64);
        }
    }
    if num > 0 {
        str.push((now, num));
    }

    let total = c % str.iter().map(|&(_, count)| count).sum::<i64>();

    let mut sum = 0;
    for (ch, count) in str {
        if total < count + sum {
            write!(out, "{}", ch).unwrap();
            return;
        }
        sum += count;
    }
}
