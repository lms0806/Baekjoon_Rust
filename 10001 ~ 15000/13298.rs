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
        writeln!(
            out,
            "{}",
            solve(
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>()
            )
        )
        .unwrap();
    }
}

fn solve(l1: i64, a1: i64, l2: i64, a2: i64, lt: i64, at: i64) -> String {
    let (mut a, mut b) = (0, 0);
    for i in 1..=lt / l1 {
        let (x, y) = (lt - (l1 * i), at - (a1 * i));
        if x % l2 == 0 && y % a2 == 0 && x / l2 == y / a2 && y / a2 > 0 {
            if a + b == 0 {
                a = i;
                b = (lt - (l1 * i)) / l2;
            } else {
                return "?".to_string();
            }
        }
    }

    if a + b == 0 {
        "?".to_string()
    } else {
        format!("{} {}", a, b)
    }
}
