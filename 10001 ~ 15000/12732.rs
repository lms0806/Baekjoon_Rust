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

    for t in 1..=scan.token::<usize>() {
        let n = scan.token::<i64>();

        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        let mut arr = Vec::with_capacity(a + b);
        for _ in 0..a {
            let (start, end) = (
                change(scan.token::<String>()),
                change(scan.token::<String>()),
            );
            arr.push((start, end, 0));
        }
        for _ in 0..b {
            let (start, end) = (
                change(scan.token::<String>()),
                change(scan.token::<String>()),
            );
            arr.push((start, end, 1));
        }

        arr.sort_unstable();

        let (mut pq_a, mut pq_b) = (BinaryHeap::new(), BinaryHeap::new());
        let (mut need_a, mut need_b) = (0, 0);

        for (start, end, flag) in arr {
            if flag == 0 {
                match pq_a.peek() {
                    Some(&Reverse(time)) if time <= start => {
                        pq_a.pop();
                    }
                    None | Some(_) => {
                        need_a += 1;
                    }
                }
                pq_b.push(Reverse(end + n));
            } else {
                match pq_b.peek() {
                    Some(&Reverse(time)) if time <= start => {
                        pq_b.pop();
                    }
                    None | Some(_) => {
                        need_b += 1;
                    }
                }
                pq_a.push(Reverse(end + n));
            }
        }
        writeln!(out, "Case #{}: {} {}", t, need_a, need_b).unwrap();
    }
}

fn change(s: String) -> i64 {
    let mut str = s.split(":");
    let hh: i64 = str.next().unwrap().parse().unwrap();
    let mm: i64 = str.next().unwrap().parse().unwrap();
    hh * 60 + mm
}
