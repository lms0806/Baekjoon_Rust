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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut board = vec![vec![0; m]; n];

    for i in 0..n {
        let str = scan.token::<String>();
        let s = str.as_bytes();
        for j in 0..m {
            board[i][j] = if s[j] == b'W' { 1 } else { 0 };
        }
    }

    let mut answer = i64::MAX;

    for i in 0..(n - 7) {
        for j in 0..(m - 7) {
            answer = answer.min(solve(&board, i, j));
        }
    }
    write!(out, "{}", answer);
}

fn solve(board: &Vec<Vec<i64>>, n: usize, m: usize) -> i64 {
    let mut color = board[n][m];

    let mut count = 0;
    for i in n..(n + 8) {
        for j in m..(m + 8) {
            if board[i][j] != color {
                count += 1;
            }

            color = if color == 0 { 1 } else { 0 };
        }
        color = if color == 0 { 1 } else { 0 };
    }

    return count.min(64 - count);
}
