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

    loop {
        let n = scan.token::<usize>();
        if n == 0 {
            break;
        }

        let mut queue = VecDeque::new();
        let (mut visited, mut number_digit, mut before) =
            (vec![false; n], vec![' '; n], vec![0; n]);

        let start = 1 % n;
        queue.push_back(start);
        visited[start] = true;
        number_digit[start] = '1';

        while let Some(now) = queue.pop_front() {
            if now == 0 {
                break;
            }

            for &(digit, add) in &[('0', 0), ('1', 1)] {
                let next = (now * 10 + add) % n;

                if !visited[next] {
                    visited[next] = true;
                    before[next] = now;
                    number_digit[next] = digit;
                    queue.push_back(next);
                }
            }
        }

        let mut now = 0;
        let mut result = vec![];
        while now != start {
            result.push(number_digit[now]);
            now = before[now];
        }
        result.push('1');

        writeln!(out, "{}", result.iter().rev().collect::<String>()).unwrap();
    }
}
