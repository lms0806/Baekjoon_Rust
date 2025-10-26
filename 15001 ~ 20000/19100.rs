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

    let (n, t) = (scan.token::<usize>(), scan.token::<usize>());
    let mut arr = (0..(1 << n))
        .map(|_| scan.token::<i64>())
        .collect::<Vec<_>>();

    if t % 2 == 1 {
        arr = shuffle(arr);
    }

    for a in arr {
        write!(out, "{} ", a).unwrap();
    }
}

fn shuffle(arr: Vec<i64>) -> Vec<i64> {
    let mid = arr.len() >> 1;
    let mut b = arr[..mid].to_vec();
    if b.len() > 2 {
        b = shuffle(b);
    } else if b.len() == 2 {
        b.swap(0, 1);
    }

    let mut c = arr[mid..].to_vec();
    if c.len() > 2 {
        c = shuffle(c);
    } else if c.len() == 2 {
        c.swap(0, 1);
    }

    let mut d = Vec::with_capacity(arr.len());
    d.extend_from_slice(&c);
    d.extend_from_slice(&b);

    d
}
