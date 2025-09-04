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

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();

        writeln!(
            out,
            "{}",
            solve(
                (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>(),
                n as i64
            )
        )
        .unwrap();
    }
}

fn solve(arr: Vec<i64>, n: i64) -> &'static str {
    let mut prev = 0;

    for x in arr {
        let (a, b) = (x, n - x + 1);

        let mut vec = vec![];
        if a >= prev {
            vec.push(a);
        }
        if b >= prev {
            vec.push(b);
        }

        if vec.is_empty() {
            return "NO";
        }

        prev = *vec.iter().min().unwrap();
    }
    "YES"
}
