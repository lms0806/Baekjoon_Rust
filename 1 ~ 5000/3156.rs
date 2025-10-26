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

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).expect("Failed read");

        if bytes_read == 0 {
            None
        } else {
            Some(input)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut map: HashMap<String, i64> = HashMap::new();
    for _ in 0..scan.token::<usize>() {
        let (a, _, b) = (
            scan.token::<usize>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );

        for _ in 0..a {
            map.entry(scan.token::<String>())
                .and_modify(|now| *now = (*now).min(b))
                .or_insert(b);
        }
    }

    let mut rank = map.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();

    rank.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    for (i, (idx, song)) in rank.iter().enumerate() {
        if i == 0 || idx != &rank[i - 1].0 {
            if idx - 1 == i as i64 {
                writeln!(out, "{} {}", idx, song).unwrap();
            }
        }
    }
}
