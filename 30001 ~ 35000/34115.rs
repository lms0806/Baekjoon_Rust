use io::Write;
use std::collections::HashMap;
use std::ptr::write;
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

    let arr = (0..scan.token::<usize>() << 1)
        .map(|_| scan.token::<usize>())
        .collect::<Vec<usize>>();

    let mut answer = 0;
    let mut map = HashMap::new();
    for (idx, &value) in arr.iter().enumerate() {
        if let Some(&start) = map.get(&value) {
            answer = answer.max(idx - start - 1);
        } else {
            map.insert(value, idx);
        }
    }
    write!(out, "{}", answer).unwrap();
}
