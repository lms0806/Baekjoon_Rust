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

    let test = scan.token::<usize>();

    for t in 1..=test {
        let (r, p, d) = (
            scan.token::<usize>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );

        let mut main = 0.0;
        let mut arr = Vec::new();
        for _ in 0..r {
            let (name, weight, percent) = (
                scan.token::<String>(),
                scan.token::<f64>(),
                scan.token::<f64>(),
            );

            if percent == 100.0 {
                main = weight;
            }

            arr.push((name, percent));
        }

        main = main * (d / p);

        writeln!(out, "Recipe # {}", t).unwrap();
        for num in arr {
            writeln!(out, "{} {:.1}", num.0, (num.1 / 100.0) * main).unwrap();
        }
        writeln!(out, "----------------------------------------").unwrap();
    }
}
