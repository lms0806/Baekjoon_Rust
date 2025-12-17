use std::cmp::Reverse;
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
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    for t in 1..=scan.token::<usize>() {
        let mut pq = BinaryHeap::new();

        write!(out, "Case #{}: ", t).unwrap();

        let mut answer = 0;
        for _ in 0..scan.token::<usize>() {
            let num = scan.token::<i64>();

            if num > answer {
                pq.push(Reverse(num));
            }

            while pq.len() > (answer + 1) as usize {
                pq.pop();
            }

            if pq.len() == (answer + 1) as usize && pq.peek().unwrap().0 >= answer + 1 {
                answer += 1;
            }

            write!(out, "{} ", answer).unwrap();
        }
        writeln!(out).unwrap();
    }
}
