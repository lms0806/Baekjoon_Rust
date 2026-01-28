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

    let mut t = 1;
    loop {
        let n = scan.token::<i64>();
        if n == 0 {
            break;
        }

        let (mut a, mut b) = change(&scan.token::<String>());
        for _ in 1..n {
            let (tmp_a, tmp_b) = change(&scan.token::<String>());

            let l = lcm(b, tmp_b);

            a = a * (l / b) + tmp_a * (l / tmp_b);
            b = l;
        }

        let g = gcd(a, b);
        a /= g;
        b /= g;

        write!(out, "Test {}: ", t).unwrap();
        match a {
            0 => writeln!(out, "0").unwrap(),
            _ => {
                let (w, r) = (a / b, a.abs() % b);

                match (w, r) {
                    (0, _) => writeln!(out, "{}/{}", r, b).unwrap(),
                    (_, 0) => writeln!(out, "{}", w).unwrap(),
                    _ => writeln!(out, "{},{}/{}", w, r, b).unwrap(),
                }
            }
        }

        t += 1;
    }
}

fn change(s: &str) -> (i64, i64) {
    if let Some((w, num)) = s.split_once(',') {
        let w = w.parse::<i64>().unwrap();
        let (a, b) = num.split_once('/').unwrap();
        let (a, b) = (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap());
        return (w * b + a, b);
    }

    if let Some((a, b)) = s.split_once('/') {
        return (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap());
    }

    (s.parse::<i64>().unwrap(), 1)
}
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
