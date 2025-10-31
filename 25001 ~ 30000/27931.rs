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

    let mut arr = (0..scan.token::<usize>())
        .map(|_| scan.token::<i64>())
        .collect::<Vec<_>>();

    arr.sort_unstable();

    let (mut odd, mut even) = (i64::MAX, i64::MAX);
    let (mut last_odd, mut last_even) = (-1000000001, -1000000001);

    if arr[0] % 2 == 0 {
        last_even = arr[0];
    } else {
        last_odd = arr[0];
    }

    for i in 1..arr.len() {
        if -1000000000 <= last_odd {
            let num = arr[i] - last_odd;

            if num % 2 == 0 {
                even = even.min(num);
            } else {
                odd = odd.min(num);
            }
        }

        if -1000000000 <= last_even {
            let num = arr[i] - last_even;

            if num % 2 == 0 {
                even = even.min(num);
            } else {
                odd = odd.min(num);
            }
        }

        if arr[i] % 2 == 0 {
            last_even = arr[i];
        } else {
            last_odd = arr[i];
        }
    }
    write!(
        out,
        "{} {}",
        if even == i64::MAX { -1 } else { even },
        if odd == i64::MAX { -1 } else { odd }
    )
    .unwrap();
}
