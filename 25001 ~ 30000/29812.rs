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
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (_, arr) = (
        scan.token::<usize>(),
        scan.token::<String>().chars().collect::<Vec<char>>(),
    );
    let (d, m) = (scan.token::<i64>(), scan.token::<i64>());

    let (mut h, mut y, mut u) = (0, 0, 0);
    let (mut count, mut answer) = (0, 0);
    for i in 0..arr.len() {
        if arr[i] == 'H' || arr[i] == 'Y' || arr[i] == 'U' {
            if arr[i] == 'H' {
                h += 1;
            } else if arr[i] == 'Y' {
                y += 1;
            } else if arr[i] == 'U' {
                u += 1;
            }
            if count > 0 {
                answer += (count * d).min(m + d);
                count = 0;
            }
        } else {
            count += 1;
        }
    }

    if count > 0 {
        answer += (count * d).min(m + d);
    }

    if answer == 0 {
        writeln!(out, "Nalmeok").unwrap();
    } else {
        writeln!(out, "{}", answer).unwrap();
    }

    let hyu = h.min(y.min(u));
    if hyu == 0 {
        write!(out, "I love HanYang University").unwrap();
    } else {
        write!(out, "{}", hyu).unwrap();
    }
}
