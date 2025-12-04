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
        .map(|_| scan.token::<String>().trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut start, mut end) = (Vec::new(), Vec::new());
    for i in 0..n {
        for j in 0..n {
            if arr[i][j] == '0' || arr[i][j] == '1' {
                continue;
            }

            if arr[i][j] == 'B' {
                start.push((i, j));
            } else {
                end.push((i, j));
            }
        }
    }

    let mut visited = vec![vec![vec![false; 2]; n]; n];
    let mut queue = VecDeque::new();

    let (start_dir, end_dir) = (
        if start[0].0 == start[1].0 { 0 } else { 1 },
        if end[0].0 == end[1].0 { 0 } else { 1 },
    );

    queue.push_back((start[1].0, start[1].1, start_dir, 0));
    visited[start[1].0][start[1].1][start_dir] = true;

    while let Some(now) = queue.pop_front() {
        let (x, y, dir, count) = now;

        if x == end[1].0 && y == end[1].1 && dir == end_dir {
            writeln!(out, "{}", count).unwrap();
            return;
        }

        let dx = [-1, 1, 0, 0];
        let dy = [0, 0, -1, 1];

        for i in 0..4 {
            let (nx, ny) = (x as i64 + dx[i], y as i64 + dy[i]);

            if nx < 0 || nx >= n as i64 || ny < 0 || ny >= n as i64 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            if !visited[nx][ny][dir] && check(&arr, n, nx, ny, dir) {
                visited[nx][ny][dir] = true;
                queue.push_back((nx, ny, dir, count + 1));
            }
        }

        if change(&arr, n, x, y) {
            let next_dir = 1 - dir;

            if !visited[x][y][next_dir] {
                visited[x][y][next_dir] = true;
                queue.push_back((x, y, next_dir, count + 1));
            }
        }
    }
    write!(out, "0").unwrap();
}

fn check(arr: &Vec<Vec<char>>, n: usize, x: usize, y: usize, dir: usize) -> bool {
    if dir == 0 {
        if y == 0 || y == n - 1 {
            return false;
        }
        if arr[x][y - 1] == '1' || arr[x][y] == '1' || arr[x][y + 1] == '1' {
            return false;
        }
    } else {
        if x == 0 || x == n - 1 {
            return false;
        }
        if arr[x - 1][y] == '1' || arr[x][y] == '1' || arr[x + 1][y] == '1' {
            return false;
        }
    }
    true
}

fn change(arr: &Vec<Vec<char>>, n: usize, x: usize, y: usize) -> bool {
    if x == 0 || x == n - 1 || y == 0 || y == n - 1 {
        return false;
    }

    for i in (x - 1)..=(x + 1) {
        for j in (y - 1)..=(y + 1) {
            if arr[i][j] == '1' {
                return false;
            }
        }
    }
    true
}
