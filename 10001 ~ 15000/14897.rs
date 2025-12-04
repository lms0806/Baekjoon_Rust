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

    let n = scan.token::<usize>();

    let mut arr = vec![0; n + 1];
    let mut comp = Vec::with_capacity(n);
    for i in 1..=n {
        arr[i] = scan.token::<usize>();
        comp.push(arr[i]);
    }

    comp.sort_unstable();
    comp.dedup();

    for i in 1..=n {
        arr[i] = comp.binary_search(&arr[i]).unwrap();
    }

    let len = comp.len();

    let m = scan.token::<usize>();
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

    query.sort_unstable_by(|a, b| {
        if a.block != b.block {
            a.block.cmp(&b.block)
        } else if a.block & 1 == 0 {
            a.e.cmp(&b.e)
        } else {
            b.e.cmp(&a.e)
        }
    });

    let (mut ans, mut freq) = (vec![0; m], vec![0; len]);

    let mut count = 0;
    let (mut s, mut e) = (query[0].s, query[0].e);

    for i in s..=e {
        if freq[arr[i]] == 0 {
            count += 1;
        }
        freq[arr[i]] += 1;
    }

    ans[query[0].idx] = count;

    for i in 1..m {
        let (qs, qe) = (query[i].s, query[i].e);

        while s < qs {
            freq[arr[s]] -= 1;
            if freq[arr[s]] == 0 {
                count -= 1;
            }
            s += 1;
        }
        while s > qs {
            s -= 1;
            if freq[arr[s]] == 0 {
                count += 1;
            }
            freq[arr[s]] += 1;
        }

        while e < qe {
            e += 1;
            if freq[arr[e]] == 0 {
                count += 1;
            }
            freq[arr[e]] += 1;
        }
        while e > qe {
            freq[arr[e]] -= 1;
            if freq[arr[e]] == 0 {
                count -= 1;
            }
            e -= 1;
        }

        ans[query[i].idx] = count;
    }

    for i in 0..m {
        writeln!(out, "{}", ans[i]).unwrap();
    }
}
