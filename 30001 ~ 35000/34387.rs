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

    let mut time = vec![vec![0; 24]; 7];
    let day_index = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    for _ in 0..scan.token::<usize>() {
        let (_, day) = (scan.token::<String>(), scan.token::<String>());

        let idx = day_index.iter().position(|&d| d == day).unwrap();

        for _ in 0..scan.token::<usize>() {
            time[idx][scan.token::<usize>()] += 1;
        }
    }

    let mut best = (0, 0);
    for i in 0..7 {
        for j in 0..24 {
            if time[i][j] > time[best.0][best.1] {
                best = (i, j);
            }
        }
    }

    write!(
        out,
        "Your professor should host office hours {} @ {:02}:00",
        day_index[best.0], best.1
    )
    .unwrap();
}
