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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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
    let (mut p, mut m) = (vec![], vec![]);

    for _ in 0..n {
        let num = scan.token::<i64>();

        if num > 0 {
            p.push(num);
        } else {
            m.push(num);
        }
    }

    p.sort_unstable();
    p.reverse();
    m.sort_unstable();

    let mut answer = 0;
    if p.len() > 0 {
        if p.len() % 2 == 1 {
            answer += p[p.len() - 1];
        }

        let mut i = 0;
        while i < p.len() - 1 {
            if p[i + 1] == 1 {
                answer += p[i] + p[i + 1];
            } else {
                answer += p[i] * p[i + 1];
            }
            i += 2;
        }
    }

    if m.len() > 0 {
        if m.len() % 2 == 1 {
            answer += m[m.len() - 1];
        }

        for i in (0..m.len() - 1).step_by(2) {
            answer += m[i] * m[i + 1];
        }
    }

    write!(out, "{}", answer).unwrap();
}
