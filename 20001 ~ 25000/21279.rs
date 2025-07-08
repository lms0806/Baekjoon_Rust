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

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).ok()?;
        if bytes_read == 0 {
            return None; // EOF
        }
        Some(input.trim().to_string())
    }
}

#[derive(Clone)]
struct Node {
    num: i64,
    value: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, c) = (scan.token::<usize>(), scan.token::<i64>());

    let (mut gold_x, mut gold_y): (Vec<Vec<Node>>, Vec<Vec<Node>>) =
        (vec![Vec::new(); 100_001], vec![Vec::new(); 100_001]);

    for i in 0..n {
        let (x, y, beauty) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        gold_x[y as usize].push(Node {
            num: x,
            value: beauty,
        });
        gold_y[x as usize].push(Node {
            num: y,
            value: beauty,
        });
    }

    let (mut x, mut y) = (100000, 0);
    let (mut answer, mut value, mut count) = (0, 0, 0);
    loop {
        if x < 0 || y > 100000 {
            break;
        }

        if count <= c {
            for node in gold_y[y as usize].iter() {
                if node.num <= x {
                    count += 1;
                    value += node.value;
                }
            }
            y += 1;
        } else {
            for node in gold_x[x as usize].iter() {
                if node.num <= y - 1 {
                    count -= 1;
                    value -= node.value;
                }
            }
            x -= 1;
        }

        if count <= c && answer < value {
            answer = value;
        }
    }
    write!(out, "{}", answer).unwrap();
}
