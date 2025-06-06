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

    let t = scan.token::<i32>();

    for _ in 0..t {
        let (n, m, l) = (
            scan.token::<i16>(),
            scan.token::<i16>(),
            scan.token::<i16>(),
        );

        let mut answer = m;
        for _ in 0..n {
            let num = scan.token::<i16>();

            if num != -1 {
                answer = answer.min(num);
            }
        }

        answer = l.max(m - answer);

        writeln!(
            out,
            "The scoreboard has been frozen with {} minute{} remaining.",
            answer,
            if answer == 1 { "" } else { "s" }
        )
        .unwrap();
    }
}
