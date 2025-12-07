use std::collections::HashMap;
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

    let n = scan.token::<usize>();

    let cow = [
        "Bessie",
        "Elsie",
        "Daisy",
        "Gertie",
        "Annabelle",
        "Maggie",
        "Henrietta",
    ];

    let mut map = (0..cow.len())
        .map(|idx| (cow[idx].to_string(), 0))
        .collect::<HashMap<_, _>>();

    for _ in 0..n {
        let s = scan.token::<String>();
        *map.entry(s).or_insert(0) += scan.token::<usize>();
    }

    let mut arr = map.values().collect::<Vec<_>>();
    arr.sort_unstable();

    let target = arr.clone().into_iter().find(|&x| x > arr[0]);

    if target.is_none() {
        write!(out, "Tie").unwrap();
        return;
    }

    let mut answer = Vec::new();
    for (k, v) in &map {
        if v == target.unwrap() {
            answer.push(k);
        }
    }

    if answer.len() == 1 {
        write!(out, "{}", answer[0]).unwrap();
    } else {
        write!(out, "Tie").unwrap();
    }
}
