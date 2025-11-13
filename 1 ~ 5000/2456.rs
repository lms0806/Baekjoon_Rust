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

    let mut arr = vec![0; 3];
    let (mut count2, mut count3) = (vec![0; 3], vec![0; 3]);
    for _ in 0..scan.token::<usize>() {
        for i in 0..3 {
            let score = scan.token::<i64>();

            arr[i] += score;

            if score == 2 {
                count2[i] += 1;
            } else if score == 3 {
                count3[i] += 1;
            }
        }
    }

    let max = *arr.iter().max().unwrap();
    let can = (0..3)
        .filter(|&i| arr[i] == max)
        .map(|i| (i, count2[i], count3[i]))
        .collect::<Vec<_>>();

    let answer = if can.len() == 1 {
        can[0].0 + 1
    } else {
        let max_can = can.iter().max_by_key(|&(_, c2, c3)| (c3, c2)).unwrap();

        let tie = can
            .iter()
            .filter(|&&(_, c2, c3)| (c3, c2) == (max_can.2, max_can.1))
            .count();

        if tie == 1 {
            max_can.0 + 1
        } else {
            0
        }
    };

    write!(out, "{} {}", answer, max).unwrap();
}
