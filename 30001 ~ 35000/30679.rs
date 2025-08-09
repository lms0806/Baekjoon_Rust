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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let arr = (0..n)
        .map(|_| (0..m).map(|_| scan.token::<i64>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let vec: Vec<usize> = (0..n)
        .filter(|&i| check(&arr, n, m, i))
        .map(|i| i + 1)
        .collect();

    writeln!(out, "{}", vec.len()).unwrap();
    for v in vec {
        write!(out, "{} ", v).unwrap();
    }
}

fn check(arr: &Vec<Vec<i64>>, n: usize, m: usize, start_row: usize) -> bool {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let (mut x, mut y) = (start_row as i64, 0);

    for i in 0..40000 {
        let val = arr[x as usize][y as usize];
        let (dx, dy) = directions[i % 4];
        x += dx * val;
        y += dy * val;

        if x < 0 || x >= n as i64 || y < 0 || y >= m as i64 {
            return false;
        }
    }

    true
}
