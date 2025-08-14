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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let arr = (0..5).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
    let mut t = scan.token::<i64>();

    let mut idx = 0;
    let (mut green, mut yellow, mut red) = (0, 0, 0);
    while t > 0 {
        let min = t.min(arr[idx]);

        match idx {
            0 => green += min,
            1 => green += min >> 1,
            2 => yellow += min,
            3 => red += min,
            _ => {
                yellow += min;
                red += min;
            }
        }

        idx = (idx + 1) % 5;
        t -= min;
    }
    write!(out, "{} {} {}", red, yellow, green).unwrap();
}
