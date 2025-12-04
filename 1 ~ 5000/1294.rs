use std::cmp::{Ordering, Reverse};
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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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

#[derive(Debug, Eq, PartialEq)]
struct Node {
    str: String,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.str.clone() + &self.str).cmp(&(self.str.clone() + &other.str))
    }
}

// `PartialOrd`도 구현해야 합니다. `Ord`를 구현하면 `PartialOrd`가 자동으로 요구됩니다.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut pq = (0..scan.token::<usize>())
        .map(|_| Node {
            str: scan.token::<String>(),
        })
        .collect::<BinaryHeap<_>>();

    let mut answer = String::new();
    while let Some(node) = pq.pop() {
        answer.push(node.str.chars().next().unwrap());

        if node.str.len() > 1 {
            pq.push(Node {
                str: node.str[1..].to_string(),
            });
        }
    }
    write!(out, "{}", answer).unwrap();
}
