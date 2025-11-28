use io::Write;
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

    let (s, e) = (
        change(&scan.token::<String>()),
        change(&scan.token::<String>()),
    );

    if s > e {
        write!(out, "0").unwrap();
        return;
    }

    let time = scan.token::<i64>() * (100 - scan.token::<i64>()) / 100;

    write!(out, "{}", if s + time <= e { 1 } else { 0 }).unwrap();
}

fn change(time: &str) -> i64 {
    let parts: Vec<i64> = time.split(':').map(|x| x.parse::<i64>().unwrap()).collect();

    let (h, m, s) = (parts[0], parts[1], parts[2]);
    h * 3600 + m * 60 + s
}
