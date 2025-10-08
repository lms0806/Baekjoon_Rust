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

    let (mut bmod, mut bcheck) = (1, 1);
    const MOD: u64 = 1_000_000_007;
    for _ in 0..scan.token::<usize>() {
        let b = scan.token::<u64>();

        bmod = (bmod * 2) % MOD;
        bmod = (bmod - (b % MOD) + MOD) % MOD;

        bcheck = if bcheck > u64::MAX / 2 {
            u64::MAX
        } else {
            bcheck * 2
        };

        if bcheck < b {
            write!(out, "error").unwrap();
            return;
        }

        if bcheck < u64::MAX {
            bcheck -= b;
        }
    }
    write!(out, "{}", bmod).unwrap();
}
