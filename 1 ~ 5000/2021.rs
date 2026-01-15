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
            buf_str: Vec::with_capacity(1 << 16),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    #[inline(always)]
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
            };
        }
    }

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok()?);
            }
            self.buf_str.clear();
            let n = self.reader.read_until(b'\n', &mut self.buf_str).ok()?;
            if n == 0 {
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_iter = "".split_ascii_whitespace();
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let (_n, l) = (scan.token::<usize>(), scan.token::<usize>());

    let (mut node, mut arr) = (vec![vec![]; 100001], vec![vec![]; 100001]);
    for i in 1..=l {
        loop {
            let num = scan.token::<i64>();
            if num == -1 {
                break;
            }

            arr[i].push(num as usize);
            node[num as usize].push(i);
        }
    }

    let (start, end) = (scan.token::<usize>(), scan.token::<usize>());

    let mut answer = i64::MAX;
    let mut queue = VecDeque::new();
    let mut visited = vec![false; 100001];

    for i in node[start].iter() {
        queue.push_back((*i, 0));
        visited[*i] = true;
    }

    while let Some((now, depth)) = queue.pop_front() {
        for &next in &arr[now] {
            if next == end {
                answer = answer.min(depth);
                break;
            }

            for i in node[next].iter() {
                if visited[*i] {
                    continue;
                }

                visited[*i] = true;
                queue.push_back((*i, depth + 1));
            }
        }
    }

    write!(out, "{}", if answer == i64::MAX { -1 } else { answer }).unwrap();
}
