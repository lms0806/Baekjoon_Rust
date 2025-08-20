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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    write!(
        out,
        "{}",
        solve(
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>()
        )
    )
    .unwrap();
}

fn solve(a: i64, b: i64, c: i64) -> &'static str {
    let num = b * b - 4 * a * c;

    if num < 0 {
        return "둘다틀렸근";
    }

    let sqrt = (num as f64).sqrt() as i64;

    if sqrt * sqrt != num || (-b - sqrt) % (2 * a) != 0 || (-b + sqrt) % (2 * a) != 0 {
        return "둘다틀렸근";
    }

    let (x, y) = ((-b - sqrt) / (2 * a), (-b + sqrt) / (2 * a));

    if x == y {
        "둘다틀렸근"
    } else if x > 1 && y > 1 && (x & (x - 1)) == 0 && (y & (y - 1)) == 0 {
        "이수근"
    } else {
        "정수근"
    }
}
