use io::Write;
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

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let arr = (0..m).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
    let mut visited = vec![false; m];
    let mut answer = i64::MAX;

    dfs(0, &arr, &mut visited, &mut answer, n);

    if answer == i64::MAX {
        write!(out, "IMPOSSIBLE").unwrap();
    } else {
        write!(out, "{}", answer).unwrap();
    }
}

fn dfs(sum: i64, arr: &[i64], visited: &mut [bool], answer: &mut i64, n: i64) {
    if n <= sum {
        *answer = (*answer).min(sum);
        return;
    }

    for i in 0..arr.len() {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        dfs(arr[i] + sum, arr, visited, answer, n);
        visited[i] = false;
    }
}
