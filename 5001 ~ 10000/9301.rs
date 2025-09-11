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

    let test = scan.token::<usize>();

    for t in 1..(test + 1) {
        let n = scan.token::<usize>();
        let mut board = vec![vec!['.'; n]; n];

        let q = scan.token::<usize>();

        for _ in 0..q {
            let op = scan.token::<String>();
            let (x, y, w, h) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<usize>(),
            );
            let ch = scan.token::<char>();

            let (sx, sy, ex, ey) = (n - (y + h - 1), x - 1, n - y, x + w - 2);

            for i in sx..=ex {
                for j in sy..=ey {
                    if op == "Filled" || i == sx || i == ex || j == sy || j == ey {
                        board[i][j] = ch;
                    }
                }
            }
        }

        writeln!(out, "Case {}:", t).unwrap();
        for value in board {
            writeln!(out, "{}", value.iter().collect::<String>()).unwrap();
        }
    }
}
