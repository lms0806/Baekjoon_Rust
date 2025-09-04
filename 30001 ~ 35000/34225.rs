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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut arr = (0..n)
        .map(|i| (scan.token::<usize>(), i + 1))
        .collect::<Vec<_>>();

    arr.sort_by_key(|x| x.0);

    let (mut max, mut sum) = (0, 0);
    let mut idx = 0;
    for i in (0..n).rev() {
        sum += arr[i].0;

        let num = arr[n - 1].0 + arr[i].0 + sum;

        if num > max {
            idx = i;
            max = num;
        }
    }

    let answer = arr[idx..n].iter().map(|&(_, idx)| idx).collect::<Vec<_>>();

    writeln!(out, "{}", answer.len()).unwrap();
    for a in answer {
        write!(out, "{} ", a).unwrap();
    }
}
