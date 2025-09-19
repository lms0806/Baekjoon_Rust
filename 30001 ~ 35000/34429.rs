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

    let (mut arr, mut d) = (
        scan.token::<String>()
            .split(":")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
        scan.token::<String>(),
    );

    if arr[0] == 12 && d == "AM" {
        arr[0] = 0;
    } else if arr[0] != 12 && d == "PM" {
        arr[0] += 12;
    }

    let time = ((arr[0] * 60 + arr[1] - scan.token::<i64>()) % 1440 + 1440) % 1440;

    let (mut h, m) = (time / 60, time % 60);
    if h < 12 {
        d = "AM".to_string();
        if h == 0 {
            h = 12;
        }
    } else {
        d = "PM".to_string();
        if h > 12 {
            h -= 12;
        }
    }
    write!(out, "{}:{:02} {}", h, m, d).unwrap();
}
