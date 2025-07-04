use io::Write;
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

    let n = scan.token::<usize>();

    let mut arr = vec![(0, 0); n];

    for _ in 0..n {
        arr.push((scan.token::<i64>(), scan.token::<i64>()));
    }

    arr.sort();

    let (l, mut p) = (scan.token::<i64>(), scan.token::<i64>());

    let mut pq: BinaryHeap<(i64, i64)> = BinaryHeap::new();

    let mut now = p;
    let (mut idx, mut answer) = (0, 0);
    loop {
        while idx < arr.len() && arr[idx].0 <= now {
            pq.push((arr[idx].1, arr[idx].0));
            idx += 1;
        }

        if now >= l {
            break;
        }

        if pq.is_empty() {
            answer = -1;
            break;
        }

        let (len, oil) = pq.pop().unwrap();

        p += oil - len;
        now += len;
        answer += 1;
    }
    write!(out, "{}", answer).unwrap();
}
