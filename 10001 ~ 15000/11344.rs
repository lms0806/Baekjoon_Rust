use std::collections::VecDeque;
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
        let (mut line_a, mut line_b) = (
            (0..scan.token::<usize>())
                .map(|_| (scan.token::<String>(), scan.token::<i64>()))
                .collect::<VecDeque<_>>(),
            VecDeque::new(),
        );

        let (mut answer, mut max_len, mut five) = (VecDeque::new(), 0, 0);
        while !line_a.is_empty() || !line_b.is_empty() {
            if five > 0 && !line_b.is_empty() {
                line_b.pop_front();
                five -= 1;
            } else if !line_a.is_empty() {
                let (name, money) = line_a.pop_front().unwrap();

                if money == 20 {
                    if five > 0 {
                        five -= 1;
                    } else {
                        line_b.push_back(name);
                    }
                } else {
                    five += 1;
                }
            } else {
                break;
            }

            if max_len < line_b.len() {
                max_len = line_b.len();
                answer = line_b.clone();
            }
        }

        if max_len == 0 {
            writeln!(out, "Line B stayed empty.").unwrap();
        } else {
            writeln!(
                out,
                "{}",
                answer
                    .iter()
                    .map(|x| x.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ")
            )
            .unwrap();
        }
    }
}
