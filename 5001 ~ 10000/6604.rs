use std::collections::HashMap;
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

    let map = (0..scan.token::<usize>())
        .map(|_| {
            (
                scan.token::<char>(),
                (scan.token::<usize>(), scan.token::<usize>()),
            )
        })
        .collect::<HashMap<_, _>>();

    while let Some(str) = scan.token_eof::<String>() {
        match solve(&str.chars().collect::<Vec<_>>(), &map) {
            Some(val) => writeln!(out, "{}", val).unwrap(),
            None => writeln!(out, "error").unwrap(),
        }
    }
}

fn solve(ch: &Vec<char>, map: &HashMap<char, (usize, usize)>) -> Option<usize> {
    let mut stack = vec![];

    for &c in ch {
        if c.is_ascii_uppercase() {
            let &(x, y) = map.get(&c)?;
            stack.push((0, x, y));
        } else if c == ')' {
            let (cost1, x1, y1) = stack.pop()?;
            let (cost2, x2, y2) = stack.pop()?;

            if y2 != x1 {
                return None;
            }

            stack.push((cost1 + cost2 + (x2 * y2 * y1), x2, y1));
        }
    }

    if stack.len() == 1 {
        Some(stack[0].0)
    } else {
        None
    }
}
