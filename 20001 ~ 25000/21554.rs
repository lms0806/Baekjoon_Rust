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
    let mut arr = (0..n).map(|_| scan.token::<usize>()).collect::<Vec<_>>();

    let mut vec: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        if arr[i] == i + 1 {
            continue;
        }

        if let Some(j) = arr.iter().position(|&x| x == i + 1) {
            vec.push((i, j));
            arr[i..=j].reverse();
        }
    }

    writeln!(out, "{}", vec.len()).unwrap();
    for (i, j) in vec {
        writeln!(out, "{} {}", i + 1, j + 1).unwrap();
    }
}
