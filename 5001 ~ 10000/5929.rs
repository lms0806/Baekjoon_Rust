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

    write!(
        out,
        "{}",
        solve(
            &mut scan.token::<String>().chars().collect::<Vec<_>>(),
            &mut scan.token::<String>().chars().collect::<Vec<_>>()
        )
    )
    .unwrap();
}

fn solve(two: &mut Vec<char>, three: &mut Vec<char>) -> i64 {
    let (val2, pow2) = calc(two, 2);
    let (val3, pow3) = calc(three, 3);

    for i in 0..two.len() {
        let bit = two[i].to_digit(2).unwrap() as i64;
        let new_val2 = val2 + ((1 - bit) - bit) * pow2[i];

        for j in 0..three.len() {
            let digit = three[j].to_digit(3).unwrap() as i64;

            for new_digit in 0..3 {
                if new_digit == digit {
                    continue;
                }

                let new_val3 = val3 + (new_digit - digit) * pow3[j];

                if new_val2 == new_val3 {
                    return new_val2;
                }
            }
        }
    }
    -1
}

fn calc(chars: &Vec<char>, base: i64) -> (i64, Vec<i64>) {
    let mut val = 0;
    for &c in chars.iter() {
        val = val * base + c.to_digit(base as u32).unwrap() as i64;
    }

    let mut p = 1;
    let mut pow = vec![];
    for _ in 0..chars.len() {
        pow.push(p);
        p *= base;
    }
    pow.reverse();

    (val, pow)
}
