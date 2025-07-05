use io::Write;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{collections::VecDeque, io, str};

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

#[derive(Eq, PartialEq, Clone, Debug)]
struct Node {
    x: i64,
    y: i64,
    idx: usize,
    flag: bool,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => match self.y.cmp(&other.y) {
                Ordering::Equal => other.idx.cmp(&self.idx),
                other => other,
            },
            other => other,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut queue: Vec<VecDeque<Node>> = vec![VecDeque::new(); m];
    for i in 0..n {
        queue[i % m].push_back(Node {
            x: scan.token::<i64>(),
            y: scan.token::<i64>(),
            idx: i % m,
            flag: i == k,
        });
    }

    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    for i in 0..m {
        if queue[i].len() > 0 {
            pq.push(queue[i].pop_front().unwrap());
        }
    }

    let mut answer = 0;
    for _ in 0..n {
        let node = pq.pop().unwrap();

        if node.flag {
            break;
        }

        if queue[node.idx].len() > 0 {
            pq.push(queue[node.idx].pop_front().unwrap());
        }

        answer += 1;
    }

    write!(out, "{}", answer).unwrap();
}
