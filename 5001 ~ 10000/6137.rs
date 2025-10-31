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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let arr = (0..scan.token::<usize>())
        .map(|_| scan.token::<char>())
        .collect::<Vec<_>>();

    let mut answer = String::new();
    let (mut s, mut e) = (0, arr.len() - 1);
    while s <= e {
        if arr[s] < arr[e] {
            answer.push(arr[s]);
            s += 1;
        } else if arr[s] > arr[e] {
            answer.push(arr[e]);
            e -= 1;
        } else {
            let (mut ss, mut ee) = (s + 1, e - 1);

            let mut front = true;
            while ss <= ee {
                if arr[ss] < arr[ee] {
                    break;
                } else if arr[ss] > arr[ee] {
                    front = false;
                    break;
                } else {
                    ss += 1;
                    ee -= 1;
                }
            }

            if front {
                answer.push(arr[s]);
                s += 1;
            } else {
                answer.push(arr[e]);
                e -= 1;
            }
        }
    }

    for chunk in answer.as_bytes().chunks(80) {
        writeln!(out, "{}", String::from_utf8_lossy(chunk)).unwrap();
    }
}
