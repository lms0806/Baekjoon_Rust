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

    let n = scan.token::<usize>();
    let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    for a in 1..=arr[0] / 3 {
        for b in (a + 1)..arr[n - 1] {
            for &num in &arr {
                if num <= a + b || num - a - b <= b {
                    continue;
                }

                let c = num - a - b;

                let mut sum = vec![
                    3 * a,
                    2 * a + b,
                    a + 2 * b,
                    3 * b,
                    2 * a + c,
                    a + b + c,
                    a + 2 * c,
                    2 * b + c,
                    b + 2 * c,
                    3 * c,
                ];

                sum.sort_unstable();
                sum.dedup();

                if sum == arr {
                    write!(out, "{} {} {}", a, b, c).unwrap();
                    return;
                }
            }
        }
    }
}
