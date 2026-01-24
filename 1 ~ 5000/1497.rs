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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut info = vec![0; n];

    for i in 0..n {
        let _ = scan.token::<String>();

        let ch = scan.token::<String>().chars().collect::<Vec<_>>();
        for j in 0..m {
            if ch[j] == 'Y' {
                info[i] |= 1 << j;
            }
        }
    }

    let mut result = (0u32, usize::MAX);
    dfs(0, n, &info, 0, 0, &mut result);

    if result.0 == 0 {
        write!(out, "-1").unwrap();
    } else {
        write!(out, "{}", result.1).unwrap();
    }
}

fn dfs(idx: usize, n: usize, info: &Vec<i64>, count: usize, sum: i64, result: &mut (u32, usize)) {
    if idx == n {
        let bit_count = sum.count_ones();

        if bit_count > result.0 {
            result.0 = bit_count;
            result.1 = count;
        } else if bit_count == result.0 && bit_count > 0 {
            if count < result.1 {
                result.1 = count;
            }
        }
        return;
    }

    dfs(idx + 1, n, info, count + 1, sum | info[idx], result);

    dfs(idx + 1, n, info, count, sum, result);
}
