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

    let n = scan.token::<usize>();
    let mut arr = vec![(0, 0); n + 1];

    for i in 1..=n {
        arr[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    arr.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut sum = vec![0; n + 1];
    sum[1] = arr[1].1;
    for i in 2..=n {
        sum[i] = arr[i].1 + sum[i - 1];
    }

    let mut idx = 1;
    let mut result = vec![0; n + 1];

    result[1] = sum[1];

    for i in 2..=n {
        result[i] = (sum[i] - sum[idx - 1]) - (arr[i].0 - arr[idx].0);

        if result[i] <= arr[i].1 {
            result[i] = arr[i].1;
            idx = i;
        }
    }

    write!(out, "{}", result.iter().max().unwrap()).unwrap();
}
