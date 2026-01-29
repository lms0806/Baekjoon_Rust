use std::collections::{HashMap, HashSet};
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

    let mut t = 1;
    loop {
        let mut vec = vec![];
        let mut map = HashMap::new();

        loop {
            let line = scan.line();
            if line == "#" {
                break;
            }

            let arr = line.split_ascii_whitespace().collect::<Vec<_>>();
            vec.push(arr[0].to_string());
            map.insert(arr[0].to_string(), arr[1].to_string());
        }

        if map.is_empty() {
            break;
        }

        writeln!(out, "Party number {}", t).unwrap();
        t += 1;

        let mut visited = HashSet::new();

        for start in vec {
            if visited.contains(&start) {
                continue;
            }

            let mut now = start.clone();

            write!(out, "{}", start).unwrap();
            loop {
                visited.insert(now.clone());
                now = map[&now].clone();

                if now == start {
                    writeln!(out, " to {}.", now).unwrap();
                    break;
                } else {
                    write!(out, " to {}", now).unwrap();
                }
            }
        }
        writeln!(out).unwrap();
    }
}
