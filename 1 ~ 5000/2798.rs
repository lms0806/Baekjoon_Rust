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

    let (size, num) = (scan.token::<usize>(), scan.token::<i64>());

    let mut arr = vec![0; size];

    for i in 0..size {
        arr[i] = scan.token::<i64>();
    }

    write!(out, "{}", solve(arr, num));
}

fn solve(arr: Vec<i64>, num: i64) -> i64 {
    let mut answer = 0;

    for i in 0..(arr.len() - 2) {
        for j in (i + 1)..(arr.len() - 1) {
            for k in (j + 1)..arr.len() {
                let temp = arr[i] + arr[j] + arr[k];

                if temp == num {
                    return temp;
                }

                if answer < temp && temp < num {
                    answer = temp;
                }
            }
        }
    }
    return answer;
}
