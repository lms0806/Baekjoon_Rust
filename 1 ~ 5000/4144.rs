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

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (x, y, z) = (
            scan.token::<u32>(),
            scan.token::<u32>(),
            scan.token::<String>(),
        );

        writeln!(out, "{}", solve(i64::from_str_radix(&z, x).unwrap(), y)).unwrap();
    }
}

fn solve(mut num: i64, base: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let d = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut result = vec![];
    while num > 0 {
        result.push(d[(num % base as i64) as usize] as char);
        num /= base as i64;
    }

    result.iter().rev().collect()
}
