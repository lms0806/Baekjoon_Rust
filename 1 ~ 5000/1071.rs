use io::Write;
use std::{io, iter::repeat, str};

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

    let n = scan.token::<usize>();
    let mut count = vec![0; 1003];

    for _ in 0..n {
        count[scan.token::<usize>()] += 1;
    }

    let mut idx = 0;
    let mut answer = Vec::with_capacity(n);
    while count.iter().sum::<usize>() > 0 {
        let mut check = true;

        if count[idx] > 0 && count[idx + 1] > 0 {
            for i in idx + 2..count.len() {
                if count[i] > 0 {
                    answer.extend(repeat(idx).take(count[idx]));
                    count[idx] = 0;

                    answer.push(i);
                    count[i] -= 1;
                    check = false;
                    break;
                }
            }

            if check {
                answer.extend(repeat(idx + 1).take(count[idx + 1]));
                count[idx + 1] = 0;

                answer.extend(repeat(idx).take(count[idx]));
                count[idx] = 0;
            }
        } else {
            answer.extend(repeat(idx).take(count[idx]));
            count[idx] = 0;
        }

        idx += 1;
    }
    write!(
        out,
        "{}",
        answer
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();
}
