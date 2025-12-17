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
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = 1;
    loop {
        let (z, i, m, mut l) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        if z == 0 && i == 0 && m == 0 && l == 0 {
            break;
        }

        let mut arr = vec![-1; m];
        let mut num = 0;

        arr[l as usize] = num;

        loop {
            num += 1;
            l = (z * l + i) % m as i64;
            let idx = l as usize;

            if arr[idx] != -1 {
                writeln!(out, "Case {}: {}", t, num - arr[idx]).unwrap();
                break;
            }
            arr[idx] = num;
        }

        t += 1;
    }
}
