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
struct Query {
    idx: usize,
    s: usize,
    e: usize,
    block: usize,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();
        if n == 0 {
            break;
        }
        let m = scan.token::<usize>();

        let mut arr = vec![0; n + 1];
        for i in 1..=n {
            arr[i] = (scan.token::<i64>() + 100_001) as usize;
        }

        let sqrt_n = (n as f64).sqrt() as usize;

        let mut query: Vec<Query> = Vec::new();
        for i in 0..m {
            let (s, e) = (scan.token::<usize>(), scan.token::<usize>());
            query.push(Query {
                idx: i,
                s,
                e,
                block: s / sqrt_n,
            });
        }

        query.sort_unstable_by(|a, b| a.block.cmp(&b.block).then(a.e.cmp(&b.e)));

        let (mut answer, mut freq, mut count) = (vec![0; m], vec![0; 200_002], vec![0; 200_002]);
        let (mut s, mut e) = (1, 0);

        let mut result = 0;
        for i in 0..m {
            let (qs, qe) = (query[i].s, query[i].e);

            while e < qe {
                e += 1;
                add(arr[e], &mut freq, &mut count, &mut result);
            }
            while e > qe {
                remove(arr[e], &mut freq, &mut count, &mut result);
                e -= 1;
            }

            while s < qs {
                remove(arr[s], &mut freq, &mut count, &mut result);
                s += 1;
            }
            while s > qs {
                s -= 1;
                add(arr[s], &mut freq, &mut count, &mut result);
            }

            answer[query[i].idx] = result;
        }

        for i in 0..m {
            writeln!(out, "{}", answer[i]).unwrap();
        }
    }
}

fn add(x: usize, freq: &mut [usize], count: &mut [usize], result: &mut usize) {
    freq[count[x]] -= 1;
    count[x] += 1;
    freq[count[x]] += 1;

    if *result < count[x] {
        *result = count[x];
    }
}

fn remove(x: usize, freq: &mut [usize], count: &mut [usize], result: &mut usize) {
    freq[count[x]] -= 1;
    if freq[count[x]] == 0 && *result == count[x] {
        *result -= 1;
    }
    count[x] -= 1;
    freq[count[x]] += 1;
}
