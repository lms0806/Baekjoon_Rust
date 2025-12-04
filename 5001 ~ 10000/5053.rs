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

    for _ in 0..scan.token::<usize>() {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        let mut arr = vec![vec![]; n];

        for i in 0..n {
            let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

            arr[i].push(a);
            if a != b {
                arr[i].push(b);
            }
        }

        let mut result = vec![None; m];

        let mut check = true;
        for i in 0..n {
            let mut visited = vec![false; m];
            if !dfs(i, &arr, &mut result, &mut visited) {
                check = false;
                break;
            }
        }

        writeln!(
            out,
            "{}",
            if check {
                "successful hashing"
            } else {
                "rehash necessary"
            }
        )
        .unwrap();
    }
}

fn dfs(
    idx: usize,
    arr: &Vec<Vec<usize>>,
    result: &mut Vec<Option<usize>>,
    visited: &mut Vec<bool>,
) -> bool {
    for &book in &arr[idx] {
        if visited[book] {
            continue;
        }

        visited[book] = true;
        if result[book].is_none() || dfs(result[book].unwrap(), arr, result, visited) {
            result[book] = Some(idx);
            return true;
        }
    }
    false
}
