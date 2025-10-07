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

    let arr = scan.token::<String>().chars().collect::<Vec<char>>();
    let k = scan.token::<usize>();

    let mut answer: Option<Vec<char>> = None;
    let &min = arr.iter().min().unwrap();
    for i in 0..arr.len() {
        if arr[i] != min {
            continue;
        }

        let mut idx = i;
        let mut tmp = Vec::with_capacity(k);
        for _ in 0..k {
            tmp.push(arr[idx]);

            idx = if idx == 0 {
                1
            } else if idx == arr.len() - 1 {
                arr.len() - 2
            } else {
                if arr[idx - 1] <= arr[idx + 1] {
                    idx - 1
                } else {
                    idx + 1
                }
            };
        }

        if answer.is_none() || tmp < *answer.as_ref().unwrap() {
            answer = Some(tmp);
        }
    }
    write!(out, "{}", answer.unwrap().iter().collect::<String>()).unwrap();
}
