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
            buf_str: Vec::with_capacity(1 << 16),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    #[inline(always)]
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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok()?);
            }
            self.buf_str.clear();
            let n = self.reader.read_until(b'\n', &mut self.buf_str).ok()?;
            if n == 0 {
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_iter = "".split_ascii_whitespace();
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let mut arr = vec![(0, 0); 8];
    arr[0] = (scan.token::<i64>(), scan.token::<i64>());
    arr[7] = (scan.token::<i64>(), scan.token::<i64>());

    let mut dist = vec![vec![i64::MAX; 8]; 8];
    dist[0][7] = (arr[0].0 - arr[7].0).abs() + (arr[0].1 - arr[7].1).abs();
    dist[7][0] = dist[0][7];

    for i in (1..7).step_by(2) {
        arr[i] = (scan.token::<i64>(), scan.token::<i64>());
        arr[i + 1] = (scan.token::<i64>(), scan.token::<i64>());

        dist[i][i + 1] =
            ((arr[i].0 - arr[i + 1].0).abs() + (arr[i].1 - arr[i + 1].1).abs()).min(10);
        dist[i + 1][i] = dist[i][i + 1];
    }

    for i in 0..8 {
        for j in 0..8 {
            dist[i][j] = dist[i][j].min((arr[i].0 - arr[j].0).abs() + (arr[i].1 - arr[j].1).abs());
        }
    }

    for k in 0..8 {
        for i in 0..8 {
            for j in 0..8 {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    write!(out, "{}", dist[0][7]).unwrap();
}
