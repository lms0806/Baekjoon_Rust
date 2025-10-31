use io::Write;
use std::collections::BTreeMap;
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

    let mut map = BTreeMap::new();

    for _ in 0..scan.token::<usize>() {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        *map.entry(a).or_insert(0) += 1;
        *map.entry(b).or_insert(0) -= 1;
    }

    let (mut sum, mut max) = (0, 0);
    let (mut s, mut e) = (0, 0);
    let mut check = false;
    for (k, v) in map {
        sum += v;

        if sum > max {
            max = sum;
            s = k;
            check = true;
        } else if sum < max && check {
            e = k;
            check = false;
        }
    }
    writeln!(out, "{}", max).unwrap();
    write!(out, "{} {}", s, e).unwrap();
}
