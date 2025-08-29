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

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());

    let mut map: HashMap<String, (i32, i64)> = HashMap::new();
    for i in 0..3 * n {
        let s = scan.line();

        let entry = map.entry(s).or_insert((0, 0));
        entry.0 += 1;
        entry.1 = i;
    }

    let mut vec: Vec<(String, (i32, i64))> = map.into_iter().collect::<Vec<_>>();

    vec.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then_with(|| b.1 .1.cmp(&a.1 .1)));

    for (k, (_, _)) in vec.iter().take(m as usize) {
        write!(out, "{}", k).unwrap();
    }
}
