use std::collections::HashMap;
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

    let n = scan.token::<usize>();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let (first, last) = (scan.token::<String>(), scan.token::<String>());
        arr.push((last, first));
    }

    arr.sort_unstable();

    let mut map = HashMap::with_capacity(n);
    for (i, (last, fist)) in arr.iter().enumerate() {
        map.insert((fist, last), i);
    }

    for _ in 0..scan.token::<usize>() {
        let now_idx = map[&(&scan.token::<String>(), &scan.token::<String>())];
        let idx = (now_idx / 3) * 3;

        for i in idx..idx + 3 {
            if i != now_idx {
                writeln!(out, "{} {}", arr[i].1, arr[i].0).unwrap();
            }
        }
    }
}
