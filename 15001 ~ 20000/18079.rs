use std::collections::HashMap;
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
        let n = scan.token::<usize>();
        let arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

        let (mut left, mut right) = (HashMap::new(), HashMap::new());
        for i in 0..n {
            *right.entry(arr[i]).or_insert(0) += 1;
        }

        let mut answer = 0;
        for i in 0..n {
            let num = arr[i];

            *right.get_mut(&num).unwrap() -= 1;
            if right[&num] == 0 {
                right.remove(&num);
            }

            for (k, left_value) in left.iter() {
                let target = 2 * num - k;

                if let Some(right_value) = right.get(&target) {
                    answer += left_value * right_value;
                }
            }

            *left.entry(num).or_insert(0) += 1;
        }
        writeln!(out, "{}", answer).unwrap();
    }
}
