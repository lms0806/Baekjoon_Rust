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
    let arr = (0..n)
        .map(|_| (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; n]; n];

    let mut vec = vec![];
    for i in 0..n {
        for j in 0..n {
            if visited[i][j] || arr[i][j] == 0 {
                continue;
            }

            vec.push(bfs(i, j, n, &arr, &mut visited));
        }
    }

    let mut answer = i64::MAX;
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            for &(x1, y1) in &vec[i] {
                for &(x2, y2) in &vec[j] {
                    answer =
                        answer.min((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs());
                }
            }
        }
    }
    write!(out, "{}", answer - 1).unwrap();
}

fn bfs(
    x: usize,
    y: usize,
    n: usize,
    arr: &Vec<Vec<i64>>,
    visited: &mut Vec<Vec<bool>>,
) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    visited[x][y] = true;

    let mut result = vec![];
    result.push((x, y));

    let dx = vec![-1, 1, 0, 0];
    let dy = vec![0, 0, -1, 1];
    while let Some((x, y)) = queue.pop_front() {
        for i in 0..4 {
            let (nx, ny) = (x as i64 + dx[i], y as i64 + dy[i]);

            if nx < 0 || nx >= n as i64 || ny < 0 || ny >= n as i64 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if visited[nx][ny] || arr[nx][ny] == 0 {
                continue;
            }

            visited[nx][ny] = true;
            queue.push_back((nx, ny));
            result.push((nx, ny));
        }
    }
    result
}
