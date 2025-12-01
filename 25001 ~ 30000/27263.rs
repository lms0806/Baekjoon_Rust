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

    for _ in 0..scan.token::<usize>() {
        writeln!(out, "{}", solve(&scan.token::<String>())).unwrap();
    }
}

fn solve(s: &str) -> String {
    let mut i = 0;
    let arr = s.chars().collect::<Vec<_>>();
    let mut answer = String::new();
    while i < arr.len() {
        if i == 0 && (arr[i] == 'e' || arr[i] == 'E') {
            answer.push_str(if arr[i] == 'e' { "ae" } else { "Ae" });
            i += 1;
            continue;
        }

        if i != 0 && arr[i] == 's' {
            if i == arr.len() - 1 || arr[i + 1] != 'h' {
                answer.push_str("th");
                i += 1;
                continue;
            }
        }

        if (arr[i] == 'o' || arr[i] == 'O') && i < arr.len() - 1 && arr[i + 1] == 'o' {
            answer.push_str(if arr[i] == 'o' { "ou" } else { "Ou" });

            let mut j = i + 2;
            while j < arr.len() && arr[j] == 'o' {
                answer.push('o');
                j += 1;
            }
            i = j;
            continue;
        }

        answer.push(arr[i]);
        i += 1;
    }
    answer
}
