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

    let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

    let mut is_prime = vec![true; b + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=b {
        if is_prime[i] {
            for j in (i * 2..=b).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut answer = 0;
    for i in a..=b {
        if is_prime[prime_factors(i as i64) as usize] {
            answer += 1;
        }
    }
    write!(out, "{}", answer).unwrap();
}

fn prime_factors(mut n: i64) -> i64 {
    let mut count = 0;

    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }

    let mut p = 3;
    while p * p <= n {
        while n % p == 0 {
            count += 1;
            n /= p;
        }
        p += 2;
    }

    if n > 1 {
        count += 1;
    }

    count
}
