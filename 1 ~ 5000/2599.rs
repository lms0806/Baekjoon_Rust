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
            buf_str: Vec::with_capacity(1 << 16),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    #[inline(always)]
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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok()?);
            }
            self.buf_str.clear();
            let n = self.reader.read_until(b'\n', &mut self.buf_str).ok()?;
            if n == 0 {
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_iter = "".split_ascii_whitespace();
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let _ = scan.token::<usize>();

    let (mut boy, mut girl) = ([0; 3], [0; 3]);
    for i in 0..3 {
        boy[i] = scan.token::<i64>();
        girl[i] = scan.token::<i64>();
    }

    if boy.iter().sum::<i64>() > girl.iter().sum::<i64>() {
        write!(out, "0").unwrap();
        return;
    }

    for i in 0..=boy[0] {
        let mut answer = [[0i64; 3]; 3];

        answer[0][1] = i;
        answer[0][2] = boy[0] - answer[0][1];

        answer[2][1] = girl[1] - answer[0][1];
        answer[2][0] = boy[2] - answer[2][1];

        answer[1][0] = girl[0] - answer[2][0];
        answer[1][2] = boy[1] - answer[1][0];

        if answer[0][2] < 0
            || answer[2][1] < 0
            || answer[2][0] < 0
            || answer[1][0] < 0
            || answer[1][2] < 0
        {
            continue;
        }

        if answer[0][2] + answer[1][2] == girl[2] {
            write!(
                out,
                "1\n{} {}\n{} {}\n{} {}",
                answer[0][1], answer[0][2], answer[1][0], answer[1][2], answer[2][0], answer[2][1]
            )
            .unwrap();
            return;
        }
    }

    write!(out, "0").unwrap();
}
