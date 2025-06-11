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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let v = scan.token::<String>().chars().collect::<Vec<_>>();

    let mut i = 0;
    let mut answer = 0;
    while i < v.len() {
        if v[i] == 'c' && i < v.len() - 1 && (v[i + 1] == '=' || v[i + 1] == '-') {
            i += 1;
        } else if v[i] == 'd' && i < v.len() - 1 {
            if v[i + 1] == 'z' && i < v.len() - 2 && v[i + 2] == '=' {
                i += 2;
            } else if v[i + 1] == '-' {
                i += 1;
            }
        } else if (v[i] == 'l' || v[i] == 'n') && i < v.len() - 1 && v[i + 1] == 'j' {
            i += 1;
        } else if (v[i] == 's' || v[i] == 'z') && i < v.len() - 1 && v[i + 1] == '=' {
            i += 1;
        }

        i += 1;
        answer += 1;
    }

    write!(out, "{}", answer);
}
