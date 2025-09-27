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

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (_, _) = (scan.token::<usize>(), scan.token::<usize>());
        let mut arr = vec![(scan.token::<i64>(), scan.token::<i64>())];
        let q = scan.token::<usize>();

        for _ in 0..q {
            arr.push((scan.token::<i64>(), scan.token::<i64>()));
        }

        let mut answer = i64::MAX;
        let mut visited = vec![false; q + 1];
        dfs(0, 0, 0, &mut answer, &arr, &mut visited);

        writeln!(out, "The shortest path has length {}", answer).unwrap();
    }
}

fn dfs(
    last_idx: usize,
    visited_count: usize,
    current_dist: i64,
    min_dist: &mut i64,
    arr: &Vec<(i64, i64)>,
    visited: &mut Vec<bool>,
) {
    if visited_count == arr.len() - 1 {
        *min_dist = (*min_dist).min(
            current_dist + (arr[last_idx].0 - arr[0].0).abs() + (arr[last_idx].1 - arr[0].1).abs(),
        );
        return;
    }

    for i in 1..arr.len() {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        dfs(
            i,
            visited_count + 1,
            current_dist + (arr[last_idx].0 - arr[i].0).abs() + (arr[last_idx].1 - arr[i].1).abs(),
            min_dist,
            arr,
            visited,
        );
        visited[i] = false;
    }
}
