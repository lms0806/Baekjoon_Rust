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
    let (s, v) = (scan.token::<f64>(), scan.token::<f64>());

    let (mut mice, mut holes) = (vec![(0.0, 0.0); n], vec![(0.0, 0.0); m]);
    for i in 0..n {
        mice[i] = (scan.token::<f64>(), scan.token::<f64>());
    }
    for i in 0..m {
        holes[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    let max_dist = s * v;
    let mut arr = vec![vec![]; n];
    for i in 0..n {
        for j in 0..m {
            let (dx, dy) = (mice[i].0 - holes[j].0, mice[i].1 - holes[j].1);

            if (dx * dx + dy * dy).sqrt() <= max_dist {
                arr[i].push(j);
            }
        }
    }

    let mut result = vec![None; m];

    let mut answer = 0;
    for i in 0..n {
        let mut visited = vec![false; m];
        if dfs(i, &arr, &mut result, &mut visited) {
            answer += 1;
        }
    }
    write!(out, "{}", n - answer).unwrap();
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
