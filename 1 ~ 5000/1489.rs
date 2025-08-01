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

    let (mut a, mut b) = (
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
        (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
    );

    a.sort();
    b.sort();
    b.reverse();

    let mut answer = 0;
    for i in 0..n {
        for j in 0..n {
            if b[j] == 0 {
                continue;
            }

            if a[i] > b[j] {
                answer += 2;
                a[i] = 0;
                b[j] = 0;
                break;
            }
        }
    }

    for i in 0..n {
        if a[i] == 0 {
            continue;
        }

        for j in 0..n {
            if a[i] == b[j] {
                answer += 1;
                b[j] = 0;
                break;
            }
        }
    }

    write!(out, "{}", answer).unwrap();
}
