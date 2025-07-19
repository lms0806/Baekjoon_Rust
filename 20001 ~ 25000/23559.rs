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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.trim().to_string()
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).ok()?;
        if bytes_read == 0 {
            return None; // EOF
        }
        Some(input.trim().to_string())
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, mut x) = (scan.token::<usize>(), scan.token::<i64>());
    let mut arr = (0..n)
        .map(|_| {
            let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
            (a, b, a - b)
        })
        .collect::<Vec<_>>();

    arr.sort_by(|a, b| b.2.cmp(&a.2));

    let mut answer = 0;
    for i in 0..n {
        let (a, b, c) = arr[i];
        if c <= 0 {
            answer += b;
            x -= 1000;
        } else if x - 5000 >= (n - i - 1) as i64 * 1000 {
            answer += a;
            x -= 5000;
        } else {
            answer += b;
            x -= 1000;
        }
    }
    write!(out, "{}", answer).unwrap();
}
