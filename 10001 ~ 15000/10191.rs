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

    let mut arr = (0..9)
        .map(|_| {
            (
                scan.token::<String>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            )
        })
        .collect::<Vec<(String, i64, i64)>>();

    for i in 0..9 {
        arr[i].1 += scan.token::<i64>();
        arr[i].2 += scan.token::<i64>();
    }

    let (best_idx, _) = arr
        .iter()
        .enumerate()
        .map(|(idx, val)| (idx, val.1 as f64 / val.2 as f64))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    if best_idx < 3 {
        arr.swap(best_idx, 3);
    } else if best_idx > 3 {
        let removed_item = arr.remove(best_idx);
        arr.insert(3, removed_item);
    }

    arr.iter().for_each(|x| {
        writeln!(out, "{}", x.0).unwrap();
    });
}
