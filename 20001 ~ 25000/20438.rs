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

    let (n, k, q, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut sleep = vec![false; n + 3];
    let mut num = vec![1; n + 3];

    (0..k).for_each(|_| {
        sleep[scan.token::<usize>()] = true;
    });

    for _ in 0..q {
        let idx = scan.token::<usize>();

        if sleep[idx] {
            continue;
        }

        for j in (idx..n + 3).step_by(idx) {
            if sleep[j] {
                continue;
            }

            num[j] = 0;
        }
    }

    let mut sum = 0;
    num[2] = 0;
    for i in 3..n + 3 {
        if num[i] > 0 {
            sum += 1;
        }

        num[i] = sum;
    }

    (0..m).for_each(|_| {
        let s = scan.token::<usize>();
        let e = scan.token::<usize>();
        writeln!(out, "{}", num[e] - num[s - 1]).unwrap();
    });
}
