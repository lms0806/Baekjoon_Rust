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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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
    let (a, b) = (
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
    );

    let mut arr = (0..n).map(|idx| (a[idx], b[idx])).collect::<Vec<_>>();

    arr.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut answer = 0;
    let (mut prev_max, mut day_max) = (arr[0].1, -1);

    for i in 0..n {
        if prev_max > arr[i].0 {
            if prev_max < arr[i].1 {
                prev_max = arr[i].1;
            }

            let count = ((prev_max - arr[i].0) + 29) / 30;
            arr[i].0 += count * 30;

            answer += count;
        }

        day_max = day_max.max(arr[i].0);

        if i + 1 < n && arr[i].1 != arr[i + 1].1 {
            prev_max = day_max;
        }
    }
    write!(out, "{}", answer).unwrap();
}
