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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.trim().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut prime = vec![false; 1000001];

    prime[0] = true;
    prime[1] = true;

    for i in 2..(1000001 as f64).sqrt() as usize {
        if !prime[i] {
            for j in (i * i..1000001).step_by(i) {
                prime[j] = true;
            }
        }
    }

    let t = scan.token::<usize>();
    for _ in 0..t {
        let n = scan.token::<usize>();

        let mut answer = 0;
        for i in 2..=(n >> 1) {
            if !prime[i] && !prime[n - i] {
                answer += 1;
            }
        }
        writeln!(out, "{}", answer);
    }
}
