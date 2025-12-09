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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut map = HashMap::new();
    for _ in 0..scan.token::<usize>() {
        map.insert(scan.token::<String>(), scan.token::<f64>());
    }

    let mut i = 0;
    let mut answer = 0.0;
    let arr = scan.token::<String>().trim().chars().collect::<Vec<char>>();
    while i < arr.len() {
        let mut s = String::new();

        s.push(arr[i]);
        i += 1;

        if i < arr.len() && arr[i].is_ascii_lowercase() {
            s.push(arr[i]);
            i += 1;
        }

        let mut num = 0;
        while i < arr.len() && arr[i].is_ascii_digit() {
            num = num * 10 + arr[i].to_digit(10).unwrap();
            i += 1;
        }

        if num == 0 {
            num = 1;
        }

        answer += map[&s] * num as f64;
    }
    write!(out, "{:.2}", answer).unwrap();
}
