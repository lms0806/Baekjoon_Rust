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

    for t in 1..=scan.token::<usize>() {
        let n = scan.token::<usize>();

        let (mut map, mut prev_map) = (HashMap::new(), HashMap::new());
        for _ in 1..n {
            let (a, b) = (scan.token::<String>(), scan.token::<String>());

            map.insert(a.clone(), b.clone());
            prev_map.insert(b, true);

            prev_map.entry(a).or_insert(false);
        }

        let mut now = String::new();
        for (k, v) in prev_map.iter() {
            if !v {
                now = k.clone();
                break;
            }
        }

        writeln!(out, "Scenario #{}:", t).unwrap();
        for _ in 0..n {
            writeln!(out, "{}", now).unwrap();

            if let Some(next) = map.get(&now) {
                now = next.clone();
            } else {
                break;
            }
        }
        writeln!(out).unwrap();
    }
}
