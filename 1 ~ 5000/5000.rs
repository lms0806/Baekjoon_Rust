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
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let (current, target) = (
        (0..n).map(|_| scan.token::<usize>()).collect::<Vec<_>>(),
        (0..n).map(|_| scan.token::<usize>()).collect::<Vec<_>>(),
    );

    write!(
        out,
        "{}",
        if inversion_parity(&current, n) == inversion_parity(&target, n) {
            "Possible"
        } else {
            "Impossible"
        }
    )
    .unwrap();
}

fn update(tree: &mut [i32], mut i: usize, v: i32) {
    while i < tree.len() {
        tree[i] += v;
        i += i & (!i + 1);
    }
}

fn query(tree: &[i32], mut i: usize) -> i32 {
    let mut s = 0;
    while i > 0 {
        s += tree[i];
        i &= i - 1;
    }
    s
}

fn inversion_parity(arr: &Vec<usize>, n: usize) -> i32 {
    let mut tree = vec![0; n + 1];
    let mut parity = 0;

    for (i, &x) in arr.iter().enumerate() {
        let inversions = (i as i32) - query(&tree, x);

        parity ^= inversions & 1;
        update(&mut tree, x, 1);
    }

    parity
}
