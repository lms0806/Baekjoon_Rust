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

    let (a, b, c, d) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    write!(
        out,
        "{}",
        solve(
            (0..a).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
            (0..b).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
            (0..c).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
            (0..d).map(|_| scan.token::<i64>()).collect::<Vec<_>>()
        )
    )
    .unwrap();
}

fn solve(arra: Vec<i64>, arrb: Vec<i64>, arrc: Vec<i64>, arrd: Vec<i64>) -> String {
    let mut map = HashMap::new();
    for &na in &arra {
        for &nb in &arrb {
            map.insert(na + nb, (na, nb));
        }
    }

    for &nc in &arrc {
        for &nd in &arrd {
            let target = -(nc + nd);
            if let Some(&(na, nb)) = map.get(&target) {
                return format!("{} {} {} {}", na, nb, nc, -(na + nb + nc));
            }
        }
    }
    "".to_string()
}
