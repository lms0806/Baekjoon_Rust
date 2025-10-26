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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut arr = (0..n)
        .map(|_| (0..m).map(|_| scan.token::<i64>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..n {
        arr[i].sort_unstable();
    }

    let mut max = 0;
    let mut pq = BinaryHeap::new();
    for i in 0..n {
        pq.push(Reverse((arr[i][0], i, 0)));
        max = max.max(arr[i][0]);
    }

    let mut answer = i64::MAX;
    loop {
        let Reverse((val, i, idx)) = pq.pop().unwrap();
        answer = answer.min(max - val);

        if idx + 1 >= m {
            break;
        }

        let next_val = arr[i][idx + 1];
        pq.push(Reverse((next_val, i, idx + 1)));
        max = max.max(next_val);
    }
    writeln!(out, "{}", answer).unwrap();
}
