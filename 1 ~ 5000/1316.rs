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

    let n = scan.token::<usize>();

    let mut answer = 0;
    for _ in 0..n {
        let v = scan.token::<String>().chars().collect::<Vec<_>>();

        let mut alpha = vec![false; 26];

        alpha[(v[0] as u8 - b'a') as usize] = true;

        let mut istrue = true;
        for i in 1..v.len() {
            let idx = (v[i] as u8 - b'a') as usize;
            if alpha[idx] && v[i] != v[i - 1] {
                istrue = false;
                break;
            }

            if !alpha[idx] {
                alpha[idx] = true;
            }
        }

        if istrue {
            answer += 1;
        }
    }

    write!(out, "{}", answer);
}
