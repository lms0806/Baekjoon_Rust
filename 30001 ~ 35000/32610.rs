use io::Write;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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

#[derive(Eq, PartialEq, Debug)]
struct Node {
    name: String,
    music: Vec<String>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.music.len().cmp(&other.music.len())
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut map = HashMap::new();
    for _ in 0..n {
        map.entry(scan.token::<String>())
            .or_insert(vec![])
            .push(scan.token::<String>());
    }

    let mut pq = map
        .into_iter()
        .map(|(name, music)| Node { name, music })
        .collect::<BinaryHeap<Node>>();

    let mut before = String::new();
    while let Some(mut node) = pq.pop() {
        if node.name == before {
            if let Some(mut next) = pq.pop() {
                if let Some(value) = next.music.pop() {
                    writeln!(out, "{}", value).unwrap();
                }

                before = next.name.clone();
                if !next.music.is_empty() {
                    pq.push(next);
                }
                pq.push(node);
            }
        } else {
            if let Some(value) = node.music.pop() {
                writeln!(out, "{}", value).unwrap();
            }

            before = node.name.clone();
            if !node.music.is_empty() {
                pq.push(node);
            }
        }
    }
}
