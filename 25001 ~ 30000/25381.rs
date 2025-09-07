use io::Write;
use std::collections::VecDeque;
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

    let ch = scan.token::<String>().chars().collect::<Vec<char>>();
    let mut check = vec![false; ch.len()];

    let mut answer = 0;
    let mut queue = VecDeque::new();
    for i in 0..ch.len() {
        if ch[i] == 'B' {
            queue.push_back(i);
        } else if ch[i] == 'C' && !queue.is_empty() {
            check[*queue.front().unwrap()] = true;
            queue.pop_front();
            answer += 1;
        }
    }

    queue.clear();
    for i in 0..ch.len() {
        if check[i] {
            continue;
        }

        if ch[i] == 'A' {
            queue.push_back(i);
        } else if ch[i] == 'B' && !queue.is_empty() {
            check[*queue.front().unwrap()] = true;
            queue.pop_front();
            answer += 1;
        }
    }

    write!(out, "{}", answer).unwrap();
}
