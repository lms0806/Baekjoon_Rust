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

    let (p, t) = (scan.token::<i64>(), scan.token::<i64>());
    let mut arr = scan.token::<String>().chars().collect::<Vec<char>>();

    let num = (t - 1) / p;
    for _ in 0..num {
        let mut vec = vec![];

        for i in 0..arr.len() {
            if arr[i] == 'I' {
                if i > 0 && arr[i - 1] == 'H' {
                    vec.push(i - 1);
                }
                if i < arr.len() - 1 && arr[i + 1] == 'H' {
                    vec.push(i + 1);
                }
            }
        }

        if vec.is_empty() {
            break;
        }

        for idx in vec {
            arr[idx] = 'I';
        }
    }

    write!(
        out,
        "{}",
        if arr.iter().any(|&x| x == 'H') {
            "CURED"
        } else {
            "ALL INFECTED"
        }
    )
    .unwrap();
}
