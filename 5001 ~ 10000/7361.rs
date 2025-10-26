use std::io::Write;
use std::{io, iter, str};

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

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).expect("Failed read");

        if bytes_read == 0 {
            None
        } else {
            Some(input)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let money = (0..5)
        .map(|_| (0..5).map(|_| scan.token::<f64>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let arr = iter::once(0)
            .chain((0..n).map(|_| scan.token::<usize>() - 1))
            .chain(iter::once(0))
            .collect::<Vec<_>>();
        let mut num = scan.token::<f64>();

        for idx in arr.windows(2) {
            num *= money[idx[0]][idx[1]];
            num = (num * 100.0).round() / 100.0;
        }

        writeln!(out, "{:.2}", num).unwrap();
    }
}
