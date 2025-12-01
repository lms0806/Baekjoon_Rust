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

    let mut arr = (0..scan.token::<usize>())
        .map(|_| scan.token::<String>())
        .map(change_time)
        .filter(|&t| t >= 6 * 60 + 30 && t <= 19 * 60)
        .collect::<Vec<_>>();

    if arr.is_empty() {
        write!(out, "0").unwrap();
        return;
    }

    arr.sort_unstable();

    write!(
        out,
        "{}",
        if arr[0] <= 10 * 60 {
            if arr[arr.len() - 1] <= 16 * 60 {
                24000
            } else {
                36000
            }
        } else {
            if arr[arr.len() - 1] <= 16 * 60 {
                16800
            } else {
                24000
            }
        }
    )
    .unwrap();
}

fn change_time(time_str: String) -> i64 {
    let parts: Vec<&str> = time_str.split(':').collect();

    parts[0].parse::<i64>().unwrap() * 60 + parts[1].parse::<i64>().unwrap()
}
