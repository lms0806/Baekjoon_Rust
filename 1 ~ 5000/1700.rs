use io::Write;
use std::collections::HashSet;
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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let arr = (0..k).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    let mut answer = 0;
    let mut set = HashSet::new();
    for i in 0..k {
        if set.contains(&arr[i]) {
            continue;
        }
        if set.len() < n {
            set.insert(arr[i]);
            continue;
        }

        answer += 1;

        let (mut idx, mut max) = (n + 1, -1);
        for &s in &set {
            let mut next = n + 1;
            for j in (i + 1)..k {
                if arr[j] == s {
                    next = j;
                    break;
                }
            }

            if next == n + 1 {
                max = s;
                break;
            }

            if next > idx {
                idx = next;
                max = s;
            }
        }

        set.remove(&max);
        set.insert(arr[i]);
    }
    write!(out, "{}", answer).unwrap();
}
