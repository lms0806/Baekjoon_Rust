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

    let n = scan.token::<usize>();

    let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
    let (mut near, mut see) = (vec![150_001; n], vec![0; n]);

    let mut stack: Vec<usize> = Vec::new();
    for i in (0..n).rev() {
        while !stack.is_empty() && arr[i] >= arr[*stack.last().unwrap()] {
            stack.pop();
        }

        see[i] += stack.len();

        if let Some(value) = stack.last() {
            near[i] = *value;
        }
        stack.push(i);
    }
    stack.clear();

    for i in 0..n {
        while !stack.is_empty() && arr[i] >= arr[*stack.last().unwrap()] {
            stack.pop();
        }

        see[i] += stack.len();

        if let Some(value) = stack.last() {
            let cur = (near[i] as i64 - i as i64).abs();
            let new = (*value as i64 - i as i64).abs();

            if cur >= new {
                near[i] = *value;
            }
        }
        stack.push(i);
    }

    for i in 0..n {
        write!(out, "{}", see[i]).unwrap();
        if see[i] != 0 {
            write!(out, " {}", near[i] + 1).unwrap();
        }
        writeln!(out, "").unwrap();
    }
}
