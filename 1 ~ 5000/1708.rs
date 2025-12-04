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

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
    p: i64,
    q: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut arr = Vec::with_capacity(n);

    for _ in 0..n {
        arr.push(Point {
            x: scan.token::<i64>(),
            y: scan.token::<i64>(),
            p: 1,
            q: 0,
        });
    }

    arr.sort_unstable_by(|a, b| {
        if a.q * b.p != a.p * b.q {
            (a.q * b.p).cmp(&(b.q * a.p))
        } else if a.y != b.y {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });

    for i in 1..n {
        arr[i].p = arr[i].x - arr[0].x;
        arr[i].q = arr[i].y - arr[0].y;
    }

    arr[1..].sort_unstable_by(|a, b| {
        if a.q * b.p != a.p * b.q {
            (a.q * b.p).cmp(&(b.q * a.p))
        } else if a.y != b.y {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });

    let mut stack = vec![];
    stack.push(0);
    stack.push(1);

    let mut next = 2;
    while next < n {
        while stack.len() >= 2 {
            let (a, b) = (stack.pop().unwrap(), *stack.last().unwrap());

            if ccw(&arr[b], &arr[a], &arr[next]) > 0 {
                stack.push(a);
                break;
            }
        }
        stack.push(next);
        next += 1;
    }

    write!(out, "{}", stack.len()).unwrap();
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}
