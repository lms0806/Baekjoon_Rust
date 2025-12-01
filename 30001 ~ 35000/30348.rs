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
    let mut answer = i64::MAX;
    let mut found = false;

    for _ in 0..n {
        let num = scan.token::<i64>();

        if check(num) {
            answer = answer.min(num);
            found = true;
        }
    }

    if found {
        writeln!(out, "{}", answer).unwrap();
    } else {
        writeln!(out, "NERASTA").unwrap();
    }
}

fn check(num: i64) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();

    if bytes.len() <= 1 {
        return true;
    }

    let (mut all_same, mut increasing) = (true, true);

    for i in 1..bytes.len() {
        if bytes[i - 1] != bytes[i] {
            all_same = false;
        }
        if bytes[i - 1] >= bytes[i] {
            increasing = false;
        }

        // 둘 다 false면 더 볼 필요 없음
        if !all_same && !increasing {
            return false;
        }
    }

    true
}
