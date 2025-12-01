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

    let price_info = [
        (0, 6, 1), // 1등 전체 6자리, 1개
        (0, 3, 2), // 앞3자리, 2개
        (3, 3, 2), // 뒤3자리, 2개
        (4, 2, 1), // 뒤2자리, 1개
    ];

    let price = price_info
        .iter()
        .map(|&(_, _, count)| {
            (0..count)
                .map(|_| (scan.token::<String>(), scan.token::<i64>()))
                .collect::<Vec<(String, i64)>>()
        })
        .collect::<Vec<_>>();

    loop {
        let ticket = scan.token::<String>();
        if ticket == "-1" {
            break;
        }

        let mut answer = 0;

        for (i, &(start, len, _)) in price_info.iter().enumerate() {
            for (num, val) in &price[i] {
                if &ticket[start..start + len] == num {
                    answer += *val;
                }
            }
        }

        writeln!(out, "{}", answer).unwrap();
    }
}
