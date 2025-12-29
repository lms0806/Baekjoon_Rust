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

    let time = scan.token::<i64>() * 60 + scan.token::<i64>();

    let mut arr = vec![];
    for _ in 0..scan.token::<usize>() {
        let (a, b, c) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let start = a * 60 + b;

        let mut i = start;
        while i < 24 * 60 {
            arr.push(i);
            i += c;
        }
        arr.push(a * 60 + b + 24 * 60);
    }

    arr.sort_unstable();

    let num = arr[arr.partition_point(|&x| x < time)];

    write!(out, "{:02}:{:02}", (num / 60) % 24, num % 60).unwrap();
}
