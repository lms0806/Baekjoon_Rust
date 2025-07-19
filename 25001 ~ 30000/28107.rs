use io::Write;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
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
        input.trim().to_string()
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).ok()?;
        if bytes_read == 0 {
            return None; // EOF
        }
        Some(input.trim().to_string())
    }
}

#[derive(Eq, Debug)]
struct Node {
    idx: usize,
    num: i64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.idx == other.idx
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .num
            .cmp(&self.num)
            .then_with(|| other.idx.cmp(&self.idx))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut customer = BinaryHeap::new();

    for i in 0..n {
        let num = scan.token::<usize>();
        for _ in 0..num {
            customer.push(Node {
                idx: i,
                num: scan.token::<i64>(),
            })
        }
    }

    let mut pq = (0..m)
        .map(|_| Reverse(scan.token::<i64>()))
        .collect::<BinaryHeap<_>>();

    let mut answer = vec![0; n];
    while let Some(value) = pq.pop() {
        while !customer.is_empty() && customer.peek().unwrap().num < value.0 {
            customer.pop();
        }

        if !customer.is_empty() && customer.peek().unwrap().num == value.0 {
            answer[customer.pop().unwrap().idx] += 1;
        }
    }

    for a in answer {
        write!(out, "{} ", a).unwrap();
    }
}
