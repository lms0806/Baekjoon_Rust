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

    let t = scan.token::<usize>();

    for _ in 0..t {
        let n = scan.token::<usize>();

        let (mut shirt, mut pants, mut socks): (Vec<String>, Vec<String>, Vec<String>) =
            (Vec::new(), Vec::new(), Vec::new());
        for _ in 0..n {
            let line = scan.line();
            let tokens: Vec<&str> = line.split_whitespace().collect();

            let mut sb = String::new();
            for token in tokens {
                if token.contains("shirt") {
                    shirt.push(sb.trim().to_string());
                } else if token.contains("pants") {
                    pants.push(sb.trim().to_string());
                } else if token.contains("socks") {
                    socks.push(sb.trim().to_string());
                } else {
                    sb.push_str(token);
                    sb.push(' ');
                }
            }
        }

        let len = shirt.len().min(pants.len()).min(socks.len());
        for _ in 0..len {
            writeln!(
                out,
                "{}, {}, {}",
                shirt.pop().unwrap(),
                pants.pop().unwrap(),
                socks.pop().unwrap()
            );
        }
        writeln!(out);
    }
}
