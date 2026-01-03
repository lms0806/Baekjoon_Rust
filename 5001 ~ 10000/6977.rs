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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..scan.token::<usize>() {
        let mut arr = Vec::new();
        dfs(
            scan.token::<i64>(),
            scan.token::<i64>(),
            String::new(),
            &mut arr,
        );

        writeln!(out, "The bit patterns are").unwrap();
        for i in arr {
            writeln!(out, "{}", i).unwrap();
        }
        writeln!(out).unwrap();
    }
}

fn dfs(n: i64, k: i64, str: String, arr: &mut Vec<String>) {
    if n == 0 {
        if k == 0 {
            arr.push(str);
        }
        return;
    }

    if k > 0 {
        dfs(n - 1, k - 1, format!("{}1", str), arr);
    }

    if n > k {
        dfs(n - 1, k, format!("{}0", str), arr);
    }
}
