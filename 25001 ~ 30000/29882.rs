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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut map = HashMap::new();
    for _ in 0..scan.token::<usize>() {
        let (name, task, cost) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );

        let entry = map.entry((name, task)).or_insert(0);

        if *entry < cost {
            *entry = cost;
        }
    }

    let mut sum_map = HashMap::new();
    for ((name, _task), cost) in map {
        *sum_map.entry(name).or_insert(0) += cost;
    }

    let mut result: Vec<_> = sum_map.into_iter().collect();
    result.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    for (name, cost) in result {
        writeln!(out, "{} {}", name, cost).unwrap();
    }
}
