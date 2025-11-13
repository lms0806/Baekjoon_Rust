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

    let mut count = vec![0; 5];
    for _ in 0..scan.token::<usize>() {
        count[scan.token::<usize>()] += 1;
    }

    let mut answer = count[4];

    answer += count[3];
    count[1] = (count[1] - count[3]).max(0);

    answer += count[2] / 2;
    count[2] %= 2;

    if count[2] > 0 {
        answer += 1;
        count[2] -= 1;
        count[1] = (count[1] - 2).max(0);
    }

    if count[1] > 0 {
        answer += count[1] / 4;

        if count[1] % 4 != 0 {
            answer += 1;
        }
    }

    write!(out, "{}", answer).unwrap();
}
