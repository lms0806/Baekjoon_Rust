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
    let (mut x, mut y) = (scan.token::<f64>(), scan.token::<f64>());

    let num = 1.0 / f64::sqrt(2.0);
    for _ in 1..n {
        let (dir, dist) = (scan.token::<String>(), scan.token::<f64>());

        match dir.as_str() {
            "N" => y += dist,
            "NE" => {
                x += dist * num;
                y += dist * num;
            }
            "E" => x += dist,
            "SE" => {
                x += dist * num;
                y -= dist * num;
            }
            "S" => y -= dist,
            "SW" => {
                x -= dist * num;
                y -= dist * num;
            }
            "W" => x -= dist,
            "NW" => {
                x -= dist * num;
                y += dist * num;
            }
            _ => {}
        }
    }
    write!(out, "{:.8} {:.8}", x, y).unwrap();
}
