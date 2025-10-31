use io::Write;
use std::collections::{BTreeSet, HashMap};
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
        let (_, _) = (scan.token::<usize>(), scan.token::<usize>());

        let mut map = HashMap::new();
        let mut map2 = HashMap::new();
        let mut answer = BTreeSet::new();

        for _ in 0..scan.token::<usize>() {
            let (a, b, money) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            *map2.entry((a, b)).or_insert(0) += money;
            if *map2.get(&(a, b)).unwrap() > 2100 {
                answer.insert(a);
            }

            *map.entry(a).or_insert(0) += money;
            if *map.get(&a).unwrap() > 40000 {
                answer.insert(a);
            }
        }

        writeln!(out, "Data Set {}:", t).unwrap();
        if answer.is_empty() {
            writeln!(out, "No violations").unwrap();
        } else {
            writeln!(out, "Violators:").unwrap();
            for a in answer {
                writeln!(out, "{}", a).unwrap();
            }
        }
    }
}
