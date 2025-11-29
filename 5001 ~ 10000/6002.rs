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
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (d, p, c, f, s) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut arr = vec![];
    for _ in 0..p {
        arr.push((scan.token::<usize>(), scan.token::<usize>(), -d));
    }
    for _ in 0..f {
        arr.push((
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>() - d,
        ));
    }

    let mut dist = vec![i64::MAX; c + 1];
    dist[s] = -d;

    for _ in 1..c {
        for (x, y, z) in arr.iter() {
            if dist[*x] < i64::MAX {
                dist[*y] = dist[*y].min(dist[*x] + z);
            }
        }
    }

    let mut check = true;
    for (x, y, z) in arr.iter() {
        if dist[*x] < i64::MAX {
            if dist[*y] > dist[*x] + z {
                check = false;
                break;
            }
        }
    }

    write!(
        out,
        "{}",
        if check {
            -dist.iter().min().unwrap()
        } else {
            -1
        }
    )
    .unwrap();
}
