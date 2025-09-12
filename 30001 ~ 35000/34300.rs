use io::Write;
use std::collections::BinaryHeap;
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

    let c = scan.token::<usize>();

    if c < 5 {
        write!(out, "{}", c).unwrap();
        return;
    }

    let mut answer = 0;
    let mut pq = (0..c)
        .map(|_| scan.token::<i64>())
        .collect::<BinaryHeap<_>>();
    while pq.len() >= 5 {
        let arr = (0..5).map(|_| pq.pop().unwrap()).collect::<Vec<_>>();

        arr.into_iter()
            .filter(|&x| x > 1)
            .for_each(|x| pq.push(x - 1));

        answer += 5;
    }
    write!(out, "{}", answer + pq.len()).unwrap();
}
