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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut map = BTreeMap::new();
        for _ in 0..n {
            for i in 0..scan.token::<usize>() {
                let entry = map.entry(scan.token::<usize>()).or_insert((0, 0, 0));

                match i {
                    0 => {
                        entry.0 += 3;
                        entry.1 += 1;
                    }
                    1 => {
                        entry.0 += 2;
                        entry.2 += 1;
                    }
                    2 => entry.0 += 1,
                    _ => {}
                }
            }
        }

        let mut values = map.into_iter().collect::<Vec<_>>();
        values.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        let max = values[0].1;
        for (id, v) in values.iter() {
            if v == &max {
                write!(out, "{} ", id).unwrap();
            }
        }
        writeln!(out).unwrap();
    }
}
