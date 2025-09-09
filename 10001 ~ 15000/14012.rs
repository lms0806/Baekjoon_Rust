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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, mut m) = (scan.token::<usize>(), scan.token::<i64>());
    let vec = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
    let mut arr = vec![(0, 0); n];

    for i in 0..n {
        arr[i] = (vec[i], scan.token::<i64>());
    }

    arr.sort_by(|a, b| a.0.cmp(&b.0));

    let mut answer = m;
    for i in 0..n {
        if m - arr[i].0 >= 0 && arr[i].0 < arr[i].1 {
            m -= arr[i].0;
            answer += arr[i].1 - arr[i].0;
            m += arr[i].1;
        }
    }
    write!(out, "{}", answer).unwrap();
}
