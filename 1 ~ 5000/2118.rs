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

    let n = scan.token::<usize>();
    let mut arr = vec![0; n * 2];

    let mut sum = 0;
    for i in 0..n {
        arr[i] = scan.token::<i64>();
        sum += arr[i];
    }
    for i in n..n * 2 {
        arr[i] = arr[i - n];
    }

    let mut now = 0;
    let mut answer = 0;
    let (mut s, mut e) = (0, 0);
    while s < n {
        loop {
            now += arr[e];
            if now > sum / 2 {
                now -= arr[e];
                break;
            }
            e += 1;
        }

        answer = answer.max(now);
        now -= arr[s];
        s += 1;
    }
    write!(out, "{}", answer).unwrap();
}
