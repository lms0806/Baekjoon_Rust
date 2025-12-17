use std::collections::BinaryHeap;
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

    let n = scan.token::<usize>();
    let mut arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    arr.sort_unstable();

    let (mut num, max) = (2, arr[arr.len() - 1]);

    if num >= max {
        write!(out, "0").unwrap();
        return;
    }

    let mut idx = 0;
    let mut pq = BinaryHeap::with_capacity(n);

    while idx < n && num > arr[idx] {
        pq.push(arr[idx]);
        idx += 1;
    }

    let mut time = 0;

    loop {
        if pq.is_empty() {
            write!(out, "NIE").unwrap();
            return;
        }

        num += pq.pop().unwrap();
        time += 1;

        if num >= max {
            write!(out, "{}", time).unwrap();
            return;
        }

        while idx < n && num > arr[idx] {
            pq.push(arr[idx]);
            idx += 1;
        }
    }
}
