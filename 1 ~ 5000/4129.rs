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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map: HashMap<i64, Vec<i64>> = HashMap::new();

    for _ in 0..n {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }

    for value in map.values_mut() {
        value.sort_unstable();
    }

    for _ in 0..m {
        let target = scan.token::<i64>();

        let mut answer = 0;
        for value in map.values() {
            let (mut l, mut r) = (0, value.len() - 1);

            while l < r {
                let sum = value[l] + value[r];

                if sum == target {
                    answer += 1;
                    l += 1;
                    r -= 1;
                } else if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        writeln!(out, "{}", answer).unwrap();
    }
}
