use std::collections::{HashMap, VecDeque};
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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let s = scan.token::<String>();

    let mut map = HashMap::new();
    for _ in 1..n {
        let (a, _, b) = (
            scan.token::<String>(),
            scan.token::<char>(),
            scan.token::<String>(),
        );

        map.entry(a).or_insert_with(Vec::new).push(b);
    }

    let mut queue = VecDeque::new();
    for value in map.get(&s).unwrap() {
        queue.push_back(value);
    }

    let mut answer = 1;
    while !queue.is_empty() {
        let size = queue.len();
        answer = answer.max(size);

        for _ in 0..size {
            let now = queue.pop_front().unwrap();

            if !map.contains_key(now) {
                continue;
            }

            for next in map.get(now).unwrap() {
                queue.push_back(next);
            }
        }
    }
    write!(out, "{}", answer).unwrap();
}
