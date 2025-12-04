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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut arr = vec![vec![]; n + 1];

    for _ in 1..=m {
        let (u, v) = (scan.token::<usize>(), scan.token::<usize>());
        if u % 2 == 1 && v % 2 == 0 {
            arr[u].push(v);
        } else if v % 2 == 1 && u % 2 == 0 {
            arr[v].push(u);
        }
    }

    let mut answer = 0;
    let mut result = vec![0; n + 1];
    for i in 1..=n {
        if i % 2 == 1 {
            let mut visited = vec![false; n + 1];
            if dfs(i, &arr, &mut result, &mut visited) {
                answer += 1;
            }
        }
    }

    write!(
        out,
        "{}",
        if answer * 2 < n {
            answer * 2 + 1
        } else {
            answer * 2
        }
    )
    .unwrap();
}

fn dfs(
    idx: usize,
    arr: &Vec<Vec<usize>>,
    result: &mut Vec<usize>,
    visited: &mut Vec<bool>,
) -> bool {
    for &book in &arr[idx] {
        if visited[book] {
            continue;
        }

        visited[book] = true;
        if result[book] == 0 || dfs(result[book], arr, result, visited) {
            result[book] = idx;
            return true;
        }
    }
    false
}
