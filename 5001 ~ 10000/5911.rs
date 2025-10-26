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

    let (n, b) = (scan.token::<usize>(), scan.token::<i64>());
    let arr = (0..n)
        .map(|_| (scan.token::<i64>(), scan.token::<i64>()))
        .collect::<Vec<_>>();

    let mut answer = 0;
    for i in 0..n {
        let mut temp = arr.clone();

        temp[i].0 /= 2;

        temp.sort_unstable_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)));

        let (mut num, mut count) = (b, 0);
        for j in 0..n {
            num -= temp[j].0 + temp[j].1;

            if num < 0 {
                break;
            }

            count += 1;
        }
        answer = answer.max(count);
    }
    write!(out, "{}", answer).unwrap();
}
