use io::Write;
use std::collections::VecDeque;
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

    let size = scan.line().split_whitespace().count();
    let n = scan.token::<usize>();

    let mut deque: VecDeque<String> = (0..n).map(|_| scan.token::<String>()).collect();

    let mut check = false;
    let (mut a, mut b) = (Vec::new(), Vec::new());
    while !deque.is_empty() {
        for _ in 1..size {
            if let Some(val) = deque.pop_front() {
                deque.push_back(val);
            }
        }

        if !check {
            a.push(deque.pop_front().unwrap());
        } else {
            b.push(deque.pop_front().unwrap());
        }
        check = !check;
    }

    writeln!(
        out,
        "{}\n{}\n{}\n{}",
        a.len(),
        a.join("\n"),
        b.len(),
        b.join("\n")
    )
    .unwrap();
}
