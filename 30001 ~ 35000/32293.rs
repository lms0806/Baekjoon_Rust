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
        let _ = scan.line();

        let mut stack: Vec<char> = Vec::new();
        let v: Vec<char> = scan.token::<String>().chars().collect();
        for ch in v {
            stack.push(ch);
            solve(&mut stack);
        }

        let mut answer = String::new();
        while !stack.is_empty() {
            answer.push(stack.pop().unwrap());
        }
        writeln!(out, "{}", answer.chars().rev().collect::<String>()).unwrap();
    }
}

fn solve(stack: &mut Vec<char>) {
    if stack.len() < 3 {
        return;
    }

    let (a, b, c) = (
        stack[stack.len() - 3],
        stack[stack.len() - 2],
        stack[stack.len() - 1],
    );

    if a == 'A' && b == 'B' && c == 'B' {
        stack.pop();
        stack.pop();
        stack.pop();

        stack.push('B');
        solve(stack);
        stack.push('A');
    }
}
