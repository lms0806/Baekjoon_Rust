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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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

    let n = scan.token::<usize>();

    let (mut a, mut b) = (vec![0; 101], vec![0; 101]);
    for _ in 0..n {
        a[scan.token::<usize>()] += 1;
        b[scan.token::<usize>()] += 1;

        let mut answer = 0;
        let (mut i, mut j) = (1, 100);
        let (mut lcount, mut rcount) = (0, 0);
        while i < 101 && j > 0 {
            if a[i] <= 0 {
                i += 1;
                continue;
            }
            if b[j] <= 0 {
                j -= 1;
                continue;
            }

            if lcount == 0 {
                lcount = a[i];
            }
            if rcount == 0 {
                rcount = b[j];
            }

            answer = answer.max((i + j) as i64);

            let num = lcount.min(rcount);
            lcount -= num;
            rcount -= num;

            if lcount <= 0 {
                i += 1;
            }
            if rcount <= 0 {
                j -= 1;
            }
        }
        writeln!(out, "{}", answer).unwrap();
    }
}
