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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..scan.token::<usize>() {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let arr = (0..n)
            .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut check = vec![vec![false; m]; n];

        let mut count = 0;
        for i in 3..n {
            for j in 3..m {
                if arr[i][j] == '.'
                    && arr[i][j - 1] == '.'
                    && arr[i][j - 2] == '.'
                    && arr[i - 1][j] == '.'
                    && arr[i - 1][j - 1] == '.'
                    && arr[i - 1][j - 2] == '.'
                    && arr[i - 2][j] == '.'
                    && arr[i - 2][j - 1] == '.'
                    && arr[i - 2][j - 2] == '.'
                {
                    check[i][j] = true;
                    count += 1;
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if !check[i][j] && arr[i][j] == '.' {
                    check[i][j] = true;
                    count += 1;
                }
            }
        }

        writeln!(out, "{}", if count % 2 == 0 { "pizza" } else { "sewon" }).unwrap();
    }
}
