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

    let mut num = scan.token::<i64>();

    if num % 4763 != 0 {
        write!(out, "0").unwrap();
        return;
    }

    num /= 4763;

    let mut answer = vec![];
    for a in 0..=200 {
        for b in 0..=200 {
            if a * 508 + b * 212 == num
                || a * 508 + b * 305 == num
                || a * 108 + b * 212 == num
                || a * 108 + b * 305 == num
            {
                answer.push((a, b));
            }
        }
    }

    answer.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)));

    writeln!(out, "{}", answer.len()).unwrap();
    for (a, b) in answer {
        writeln!(out, "{} {}", a, b).unwrap();
    }
}
