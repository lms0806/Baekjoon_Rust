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
        let s = str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let line = scan.line();
        if line == "0" {
            break;
        }

        let arr = line
            .split("+")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut number = vec![vec![]; 10];
        for num in arr {
            number[(num % 10) as usize].push(num);
        }

        let mut result = vec![];
        for i in 1..5 {
            while !number[i].is_empty() && !number[10 - i].is_empty() {
                result.push(number[i].pop().unwrap());
                result.push(number[10 - i].pop().unwrap());
            }
        }

        while number[5].len() > 1 {
            result.push(number[5].pop().unwrap());
            result.push(number[5].pop().unwrap());
        }

        while number[0].len() > 1 {
            result.push(number[0].pop().unwrap());
            result.push(number[0].pop().unwrap());
        }

        for i in 0..10 {
            for num in number[i].drain(..) {
                result.push(num);
            }
        }

        for (i, x) in result.iter().enumerate() {
            if i > 0 {
                write!(out, "+").unwrap();
            }
            write!(out, "{}", x).unwrap();
        }
        writeln!(out).unwrap();
    }
}
