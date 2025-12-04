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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let mut arr = vec![0; n + 1];
    for i in 1..=n {
        arr[i] = scan.token::<usize>();
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

    let mut ans = vec![0; m];
    let mut freq = vec![0; 1000001];

    let mut count = 0;
    let (mut s, mut e) = (query[0].s, query[0].e);

    for i in s..=e {
        count -= freq[arr[i]] * freq[arr[i]] * arr[i];
        freq[arr[i]] += 1;
        count += freq[arr[i]] * freq[arr[i]] * arr[i];
    }

    ans[query[0].idx] = count;

    for i in 1..m {
        let (qs, qe) = (query[i].s, query[i].e);

        while s < qs {
            count -= freq[arr[s]] * freq[arr[s]] * arr[s];
            freq[arr[s]] -= 1;
            count += freq[arr[s]] * freq[arr[s]] * arr[s];
            s += 1;
        }
        while s > qs {
            s -= 1;
            count -= freq[arr[s]] * freq[arr[s]] * arr[s];
            freq[arr[s]] += 1;
            count += freq[arr[s]] * freq[arr[s]] * arr[s];
        }

        while e < qe {
            e += 1;
            count -= freq[arr[e]] * freq[arr[e]] * arr[e];
            freq[arr[e]] += 1;
            count += freq[arr[e]] * freq[arr[e]] * arr[e];
        }
        while e > qe {
            count -= freq[arr[e]] * freq[arr[e]] * arr[e];
            freq[arr[e]] -= 1;
            count += freq[arr[e]] * freq[arr[e]] * arr[e];
            e -= 1;
        }

        ans[query[i].idx] = count;
    }

    for i in 0..m {
        writeln!(out, "{}", ans[i]).unwrap();
    }
}
