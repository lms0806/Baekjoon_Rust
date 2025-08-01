use io::Write;
use std::collections::HashMap;
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
    let arr: Vec<(String, i64)> = (0..n)
        .map(|_| (scan.token::<String>(), scan.token::<i64>()))
        .collect();

    let m = scan.token::<usize>();
    let map: HashMap<String, i64> = (0..m)
        .map(|_| (scan.token::<String>(), scan.token::<i64>()))
        .collect();

    let q = scan.token::<usize>();

    let mut name = String::new();
    for _ in 0..q {
        let (a, b, c) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );

        if a == "nani" && b == "ga" && c == "suki?" {
            let num = map.get(&name).unwrap();

            let mut vec: Vec<(i64, &str)> = arr
                .iter()
                .map(|(algo_name, diff)| ((diff - num).abs(), algo_name.as_str()))
                .collect();

            vec.sort_by(|a, b| match a.0.cmp(&b.0) {
                std::cmp::Ordering::Equal => a.1.cmp(&b.1),
                other => other,
            });

            writeln!(out, "{} yori mo {}", vec[1].1, vec[0].1).unwrap();
        } else {
            name = a;
            writeln!(out, "hai!").unwrap();
        }
    }
}
