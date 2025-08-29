use io::Write;
use std::collections::HashSet;
use std::{cmp::max, io, str};

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

    let n = scan.token::<usize>();
    let mut arr = vec![vec![]; n];

    let mut max_len = 0;
    for i in 0..n {
        loop {
            let x = scan.token::<String>();

            if x == "-1" {
                break;
            }

            arr[i].push(x);
        }

        max_len = max(max_len, arr[i].len());
    }

    let mut answer = 0;
    for i in 1..=max_len {
        let mut set = HashSet::new();

        for j in 0..n {
            set.insert(
                arr[j]
                    .iter()
                    .take(i)
                    .map(|x| x.as_bytes())
                    .collect::<Vec<_>>(),
            );
        }

        if set.len() == n {
            answer = i;
            break;
        }
    }
    writeln!(out, "{}", answer).unwrap();
}
