use std::collections::HashSet;
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
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut enemy = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if scan.token::<i64>() == 1 {
                enemy.push((i, j));
            }
        }
    }

    let mut answer = 0;
    for i in 0..m - 2 {
        for j in i + 1..m - 1 {
            for k in j + 1..m {
                answer = answer.max(bfs(&enemy, [i, j, k], n, d));
            }
        }
    }
    write!(out, "{}", answer).unwrap();
}

fn bfs(enemy: &Vec<(i64, i64)>, archer: [i64; 3], n: i64, d: i64) -> usize {
    let mut copy_enemy = enemy.clone();
    let mut kill = 0;

    while !copy_enemy.is_empty() {
        let mut target = [None; 3];

        for i in 0..3 {
            target[i] = select_target(archer[i], &copy_enemy, n, d);
        }

        let mut dead = vec![false; copy_enemy.len()];
        for t in target.iter().flatten() {
            if !dead[*t] {
                dead[*t] = true;
                kill += 1;
            }
        }

        let mut next = Vec::with_capacity(copy_enemy.len());
        for (i, &(x, y)) in copy_enemy.iter().enumerate() {
            if !dead[i] {
                let nx = x + 1;
                if nx < n {
                    next.push((nx, y));
                }
            }
        }

        copy_enemy = next;
    }
    kill
}

fn select_target(archer_idx: i64, enemy: &Vec<(i64, i64)>, n: i64, d: i64) -> Option<usize> {
    let mut answer = None;

    for (i, &(x, y)) in enemy.iter().enumerate() {
        let dist = (n - x).abs() + (archer_idx - y).abs();

        if dist <= d {
            match answer {
                None => answer = Some((dist, y, i)),
                Some((bd, by, _)) => {
                    if dist < bd || (dist == bd && y < by) {
                        answer = Some((dist, y, i));
                    }
                }
            }
        }
    }

    answer.map(|(_, _, idx)| idx)
}
