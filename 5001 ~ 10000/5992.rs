use std::collections::VecDeque;
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
    let mut arr = vec![(0, 0); n + 1];

    for _ in 1..n {
        let (c, d1, d2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        arr[c] = (d1, d2);
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n + 1];

    queue.push_back((1, 1));
    visited[1] = true;

    let mut answer = 1;
    while let Some((now, dist)) = queue.pop_front() {
        answer = answer.max(dist);

        let (d1, d2) = arr[now];

        if !visited[d1] {
            visited[d1] = true;
            queue.push_back((d1, dist + 1));
        }
        if !visited[d2] {
            visited[d2] = true;
            queue.push_back((d2, dist + 1));
        }
    }
    write!(out, "{}", answer).unwrap();
}
