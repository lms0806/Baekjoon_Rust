use std::collections::{HashMap, HashSet};
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

    let mut map = HashMap::new();

    for _ in 0..scan.token::<usize>() {
        let x = scan.token::<char>();

        if !map.contains_key(&x) {
            map.insert(x, HashSet::new());
        }

        for _ in 0..scan.token::<usize>() {
            map.get_mut(&x).unwrap().insert(scan.token::<char>());
        }
    }

    let arr = scan.token::<String>().trim().chars().collect::<Vec<_>>();

    for i in 1..arr.len() {
        if !map.contains_key(&arr[i - 1]) || !map.get(&arr[i - 1]).unwrap().contains(&arr[i]) {
            write!(out, "no").unwrap();
            return;
        }
    }
    write!(out, "yes").unwrap();
}
