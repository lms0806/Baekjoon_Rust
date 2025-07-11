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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.trim().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.line();
    let mut str: Vec<&str> = Vec::new();

    let mut answer = 0;
    for i in 0..s.len() {
        str.push(&s[i..]);
    }

    str.sort();

    let mut answer = str[0].len();
    for i in 1..str.len() {
        answer += str[i].len() - getCommon(str[i - 1], str[i]);
    }

    write!(out, "{}", answer);
}

fn getCommon(s1: &str, s2: &str) -> usize {
    let mut answer = 0;
    let (bytes1, bytes2) = (s1.as_bytes(), s2.as_bytes());
    let size = s1.len().min(s2.len());
    for i in 0..size {
        if bytes1[i] != bytes2[i] {
            break;
        }

        answer += 1;
    }
    return answer;
}
