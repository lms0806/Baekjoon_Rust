use io::Write;
use std::cmp::Ordering;
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
        input
    }
}

struct Node {
    idx: usize,
    size: i64,
    cost: i64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (k, n, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut vec = vec![Vec::new(); n + 1];

    for _ in 0..r {
        vec[scan.token::<usize>()].push((
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        ));
    }

    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    let mut dist = vec![vec![i64::MAX; k + 1]; n + 1];
    pq.push(Node {
        idx: 1,
        size: 0,
        cost: 0,
    });

    while let Some(Node { idx, size, cost }) = pq.pop() {
        if dist[idx][cost as usize] < size {
            continue;
        }

        for &(next, l, t) in &vec[idx] {
            let next_cost = cost + t;

            if next_cost as usize > k {
                continue;
            }

            if size + l < dist[next][next_cost as usize] {
                dist[next][next_cost as usize] = size + l;
                pq.push(Node {
                    idx: next,
                    size: size + l,
                    cost: next_cost,
                });
            }
        }
    }

    let answer = dist[n].iter().min().unwrap();

    write!(out, "{}", if *answer == i64::MAX { -1 } else { *answer }).unwrap();
}
