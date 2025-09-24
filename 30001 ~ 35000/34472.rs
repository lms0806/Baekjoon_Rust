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
    let mut arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    let target = arr.iter().sum::<i64>() / n as i64;

    let mut answer = 0;

    solve(&mut arr, target, &mut answer);

    if arr[n - 1] > target {
        answer += arr[n - 1] - target;
        arr[0] += arr[n - 1] - target;
        arr[n - 1] = target;
    }

    solve(&mut arr, target, &mut answer);

    write!(out, "{}", answer).unwrap();
}

fn solve(arr: &mut [i64], target: i64, answer: &mut i64) {
    for i in 0..arr.len() - 1 {
        if arr[i] > target {
            let excess = arr[i] - target;
            *answer += excess;
            arr[i] = target;
            arr[i + 1] += excess;
        }
    }
}
