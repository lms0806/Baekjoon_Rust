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

    for t in 1..=scan.token::<usize>() {
        let money = scan.token::<i64>();

        let arr = (0..12).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

        let mut min = i64::MAX;
        let (mut a, mut b, mut answer) = (0, 0, 0);
        for i in 0..12 {
            for j in (i + 1)..12 {
                if arr[i] >= arr[j] {
                    continue;
                }

                let num = (money / arr[i]) * (arr[j] - arr[i]);

                if answer < num || (answer == num && min > arr[i]) {
                    answer = num;
                    a = i + 1;
                    b = j + 1;
                    min = arr[i];
                }
            }
        }

        if answer == 0 {
            writeln!(out, "Case #{}: IMPOSSIBLE", t).unwrap();
        } else {
            writeln!(out, "Case #{}: {} {} {}", t, a, b, answer).unwrap();
        }
    }
}
