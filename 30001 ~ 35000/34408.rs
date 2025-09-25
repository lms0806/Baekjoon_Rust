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
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (s, t) = (
        scan.token::<String>().chars().collect::<Vec<_>>(),
        scan.token::<String>().chars().collect::<Vec<_>>(),
    );
    let (mut sch, mut tch) = (vec![0; 26], vec![0; 26]);

    for &c in &s {
        sch[(c as u8 - b'A') as usize] += 1;
    }
    for &c in &t {
        tch[(c as u8 - b'A') as usize] += 1;
    }

    for i in 0..26 {
        if sch[i] < tch[i] {
            write!(out, "NEED FIX").unwrap();
            return;
        }
    }
    write!(out, "OK").unwrap();
}
