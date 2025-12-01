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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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

    let (n, k, d) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut arr = Vec::with_capacity(n);

    for _ in 0..n {
        let (m, power) = (scan.token::<usize>(), scan.token::<i64>());

        arr.push((
            power,
            (0..m).map(|_| scan.token::<usize>()).collect::<Vec<_>>(),
        ));
    }

    arr.sort_unstable();

    let (mut s, mut e) = (0, 0);
    let mut count = vec![0; k + 1];

    for &num in arr[0].1.iter() {
        count[num] += 1;
    }

    let mut answer = 0;
    loop {
        if arr[e].0 - arr[s].0 <= d {
            answer = answer.max(get_energy(&count, s, e));
            e += 1;

            if e >= n {
                write!(out, "{}", answer).unwrap();
                return;
            }

            for &num in arr[e].1.iter() {
                count[num] += 1;
            }
            continue;
        }

        for &num in arr[s].1.iter() {
            count[num] -= 1;
        }

        s += 1;
    }
}

fn get_energy(arr: &Vec<i64>, s: usize, e: usize) -> usize {
    let (mut count1, mut count2) = (0, 0);
    for i in 0..arr.len() {
        if arr[i] != 0 {
            count1 += 1;
        }
        if arr[i] == (e - s + 1) as i64 {
            count2 += 1;
        }
    }
    (count1 - count2) * (e - s + 1)
}
