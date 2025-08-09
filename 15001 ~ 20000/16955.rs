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

    write!(
        out,
        "{}",
        solve(
            (0..10)
                .map(|_| scan.token::<String>().chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        )
    )
    .unwrap();
}

fn solve(arr: Vec<Vec<char>>) -> i8 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j] != 'X' {
                continue;
            }

            for &(dx, dy) in &directions {
                let (mut count, mut empty) = (1, 0);
                let (mut x, mut y) = (i as i64, j as i64);

                for _ in 0..4 {
                    x += dx;
                    y += dy;

                    if x >= 0 && x < arr.len() as i64 && y >= 0 && y < arr[i].len() as i64 {
                        match arr[x as usize][y as usize] {
                            'X' => count += 1,
                            '.' => empty += 1,
                            _ => {}
                        }
                    }
                }

                if count == 4 && empty == 1 {
                    return 1;
                }
            }
        }
    }
    0
}
