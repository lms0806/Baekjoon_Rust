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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();
        if n == 0 {
            break;
        }

        let mut arr = (0..n)
            .map(|_| {
                (
                    change(scan.token::<String>()),
                    change(scan.token::<String>()),
                )
            })
            .collect::<Vec<_>>();

        arr.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut answer = 0;
        let mut pq = BinaryHeap::new();
        for (start, end) in arr {
            if let Some(&Reverse(time)) = pq.peek() {
                if time <= start {
                    pq.pop();
                }
            }

            pq.push(Reverse(end));

            answer = answer.max(pq.len());
        }
        writeln!(out, "{}", answer).unwrap();
    }
}

fn change(s: String) -> i64 {
    let mut str = s.split(":");
    let hh: i64 = str.next().unwrap().parse().unwrap();
    let mm: i64 = str.next().unwrap().parse().unwrap();
    let ss: i64 = str.next().unwrap().parse().unwrap();
    hh * 60 * 60 + mm * 60 + ss
}
