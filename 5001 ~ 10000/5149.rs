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
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    for t in 1..=scan.token::<usize>() {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        let arr = (0..n)
            .map(|_| (scan.token::<i64>(), scan.token::<i64>()))
            .collect::<Vec<_>>();

        let (mut minx, mut miny) = (i64::MAX, i64::MAX);
        let (mut maxx, mut maxy) = (i64::MIN, i64::MIN);
        for _ in 0..m {
            let idx = scan.token::<usize>() - 1;

            minx = minx.min(arr[idx].0);
            miny = miny.min(arr[idx].1);
            maxx = maxx.max(arr[idx].0);
            maxy = maxy.max(arr[idx].1);
        }

        let mut answer = 0;
        for i in 0..n {
            if minx <= arr[i].0 && arr[i].0 <= maxx && miny <= arr[i].1 && arr[i].1 <= maxy {
                answer += 1;
            }
        }

        writeln!(out, "Data Set {}:", t).unwrap();
        writeln!(out, "{}", answer).unwrap();
        writeln!(out).unwrap();
    }
}
