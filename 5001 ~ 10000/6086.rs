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
        let s = str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let mut arr = vec![vec![0; 52]; 52];

    for _ in 0..scan.token::<usize>() {
        let (s, e, cost) = (
            char_to_idx(scan.token::<String>().as_bytes()[0]),
            char_to_idx(scan.token::<String>().as_bytes()[0]),
            scan.token::<i64>(),
        );

        arr[s][e] += cost;
        arr[e][s] += cost;
    }

    write!(out, "{}", max_flow(0, 25, &arr)).unwrap();
}

fn char_to_idx(c: u8) -> usize {
    if b'a' <= c && c <= b'z' {
        (c - b'a' + 26) as usize
    } else {
        (c - b'A') as usize
    }
}

fn max_flow(start: usize, end: usize, arr: &Vec<Vec<i64>>) -> i64 {
    let mut answer = 0;
    let mut f = vec![vec![0; 52]; 52];
    loop {
        let mut queue = VecDeque::new();
        let mut d = vec![-1; 52];

        d[start] = start as i64;
        queue.push_back(start);

        while let Some(now) = queue.pop_front() {
            for i in 0..arr[now].len() {
                if arr[now][i] - f[now][i] > 0 && d[i] == -1 {
                    queue.push_back(i);
                    d[i] = now as i64;

                    if i == end {
                        break;
                    }
                }
            }
        }

        if d[end] == -1 {
            return answer;
        }

        let mut v = end;
        let mut flow = i64::MAX;
        while v != start {
            let u = d[v] as usize;
            flow = flow.min(arr[u][v] - f[u][v]);
            v = u;
        }

        v = end;
        while v != start {
            let u = d[v] as usize;
            f[u][v] += flow;
            f[v][u] -= flow;
            v = u;
        }

        answer += flow;
    }
}
