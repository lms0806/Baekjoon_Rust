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

    let mut all_line = vec![];
    while let Some(line) = scan.line_eof() {
        all_line.push(line);
    }

    let mut len = 0;
    for (i, line) in all_line.iter().enumerate() {
        if line.chars().filter(|&c| c == '#').count() > 1 {
            len = i;
            break;
        }
    }

    let s = all_line[len].find("#").unwrap_or(0);
    let e = all_line[len].rfind("#").unwrap_or(s);

    let mut answer = vec![];
    for col in s..=e {
        let mut count = 0;
        for i in (len + 1)..all_line.len() {
            if all_line[i].trim_end().chars().nth(col) == Some('#') {
                count += 1;
            }
        }

        if count > 0 {
            answer.push(count.to_string());
        }
    }
    writeln!(out, "{}\n{}", len, answer.join(" ")).unwrap();
}
