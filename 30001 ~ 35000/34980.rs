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

    let _ = scan.token::<usize>();

    let (s, t) = (scan.token::<String>(), scan.token::<String>());

    if s == t {
        write!(out, "Good").unwrap();
        return;
    }

    let (mut sc, mut tc) = (0, 0);
    for ch in s.chars() {
        if ch == 'w' {
            sc += 1;
        }
    }
    for ch in t.chars() {
        if ch == 'w' {
            tc += 1;
        }
    }

    let answer = ["Oryang", "Its fine", "Manners maketh man"];

    write!(
        out,
        "{}",
        answer[if sc < tc { 1 } else { 0 } + 1 - if sc > tc { 1 } else { 0 }]
    )
    .unwrap();
}
