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
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (w, n) = (scan.token::<usize>(), scan.token::<usize>());
    let arr = (0..n).map(|_| scan.token::<usize>()).collect::<Vec<_>>();

    let (mut d, mut d2) = (vec![-1; 400001], vec![-1; 400001]);

    for i in 0..n {
        for j in i + 1..n {
            let weight = arr[i] + arr[j];

            if d[weight] == -1 {
                d[weight] = i as i64;
                d2[weight] = j as i64;
            }
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if w < arr[i] + arr[j] {
                continue;
            }

            let weight = w - (arr[i] + arr[j]);

            if weight > 400000 || d[weight] < 0 {
                continue;
            }

            if d[weight] != i as i64
                && d2[weight] != i as i64
                && d[weight] != j as i64
                && d2[weight] != j as i64
            {
                write!(out, "YES").unwrap();
                return;
            }
        }
    }
    write!(out, "NO").unwrap();
}
