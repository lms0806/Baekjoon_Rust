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

    let n = scan.token::<usize>();

    let mut arr = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
    for i in 0..n {
        arr[i] -= scan.token::<i64>();
    }

    let q = scan.token::<usize>();
    let mut vec = (0..q).map(|i| (scan.token::<i64>(), i)).collect::<Vec<_>>();

    vec.sort_by(|a, b| b.0.cmp(&a.0));

    let mut answer = vec![0; q];

    let mut index = 0;
    for i in 0..q {
        while index < n && arr[index] >= vec[i].0 {
            index += 1;
        }
        answer[vec[i].1] = index;
    }

    answer.iter().for_each(|x| writeln!(out, "{}", x).unwrap());
}
