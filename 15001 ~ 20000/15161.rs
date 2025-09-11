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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut arr = vec![vec![1; 10]; 10];
    for _ in 0..n {
        arr.iter_mut().flatten().for_each(|val| *val += 1);

        for idx in (0..3).map(|_| scan.token::<usize>() - 1) {
            arr[idx].iter_mut().for_each(|val| *val = 1);
        }

        for idx in (0..3).map(|_| scan.token::<usize>() - 1) {
            arr.iter_mut().for_each(|val| val[idx] = 1);
        }
    }

    for row in &arr {
        for val in row {
            write!(out, "{} ", val).unwrap();
        }
        writeln!(out).unwrap();
    }
}
