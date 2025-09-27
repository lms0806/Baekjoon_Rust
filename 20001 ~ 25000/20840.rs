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

    let _ = scan.token::<usize>();

    let (mut a, mut b, mut wina, mut winb) = (0i64, 0i64, 0, 0);
    for ch in scan.token::<String>().chars() {
        if ch == 'A' {
            a += 1;
        } else {
            b += 1;
        }

        let num = if wina + winb < 2 { 25 } else { 15 };

        if (a - b).abs() >= 2 && (a >= num || b >= num) {
            if a > b {
                wina += 1;
            } else {
                winb += 1;
            }
            a = 0;
            b = 0;
        }

        if wina == 2 || winb == 2 {
            break;
        }
    }
    write!(out, "{} {}", wina, winb).unwrap();
}
