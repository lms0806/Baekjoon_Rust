use io::Write;
use std::cmp::Reverse;
use std::{collections::BinaryHeap, io, str};

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
        input.trim().to_string()
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).ok()?;
        if bytes_read == 0 {
            return None; // EOF
        }
        Some(input.trim().to_string())
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (k, n) = (scan.token::<usize>(), scan.token::<i64>());

    let mut arr = vec![0; k];
    let mut pq = BinaryHeap::new();
    for i in 0..k {
        arr[i] = scan.token::<i64>();
        pq.push(Reverse(arr[i]));
    }

    let (mut count, mut answer) = (0, 0i64);
    while count != n {
        while pq.peek().map(|&Reverse(x)| x).unwrap() == answer {
            pq.pop();
        }

        for i in 0..k {
            let num = arr[i] * pq.peek().unwrap().0;

            if num > i32::MAX as i64 {
                break;
            }

            pq.push(Reverse(num));
        }

        answer = pq.pop().unwrap().0;
        count += 1;
    }
    write!(out, "{}", answer).unwrap();
}
