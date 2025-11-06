use std::cmp::Reverse;
use std::collections::BTreeMap;
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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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

    let mut map: BTreeMap<Reverse<i64>, Vec<(String, bool)>> = BTreeMap::new();
    for _ in 0..scan.token::<usize>() {
        let (name, score, is_hidden) = parse(
            scan.line()
                .trim()
                .trim_end_matches(',')
                .trim_start_matches('[')
                .trim_end_matches(']'),
        );

        map.entry(Reverse(score))
            .or_insert(Vec::new())
            .push((name, is_hidden));
    }

    let mut rank = 1;
    for (score, names) in map.iter_mut() {
        names.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        names
            .iter()
            .filter(|(_, is_hidden)| !*is_hidden)
            .for_each(|(name, _)| {
                writeln!(out, "{} {} {}", rank, name, score.0).unwrap();
            });

        rank += names.len();
    }
}

fn parse(s: &str) -> (String, i64, bool) {
    let s = s.trim().trim_start_matches('{').trim_end_matches('}');

    let (mut name, mut score, mut is_hidden) = (String::new(), 0, false);
    for field in s.split(",") {
        let mut iter = field.split(':');
        let key = iter.next().unwrap().trim().trim_matches('"');
        let val = iter.next().unwrap().trim().trim_matches('"');

        match key {
            "name" => name = val.to_string(),
            "score" => score = val.parse().unwrap(),
            "isHidden" => is_hidden = val == "1",
            _ => {}
        }
    }

    (name, score, is_hidden)
}
