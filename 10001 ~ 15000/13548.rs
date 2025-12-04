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

    let mut max = 0;
    let mut arr = vec![0; n + 1];
    for i in 1..=n {
        arr[i] = scan.token::<usize>();
        max = max.max(arr[i]);
    }

    let m = scan.token::<usize>();

    let mut query: Vec<Query> = Vec::with_capacity(m);
    for i in 0..m {
        let (s, e) = (scan.token::<usize>(), scan.token::<usize>());
        query.push(Query {
            idx: i,
            s,
            e,
            block: s / (n as f64).sqrt() as usize,
        });
    }

    query.sort_unstable_by(|a, b| {
        if a.block != b.block {
            a.block.cmp(&b.block)
        } else {
            if a.block & 1 == 0 {
                a.e.cmp(&b.e)
            } else {
                b.e.cmp(&a.e)
            }
        }
    });

    let mut ans = vec![0; m];
    let mut freq = vec![0; max + 1];
    let mut freq_count = vec![0; n + 1];

    let mut count = 0;
    let (mut s, mut e) = (query[0].s, query[0].e);

    for i in s..=e {
        add_val(arr[i], &mut freq, &mut freq_count, &mut count);
    }
    ans[query[0].idx] = count;

    for i in 1..m {
        let (qs, qe) = (query[i].s, query[i].e);

        while s > qs {
            s -= 1;
            add_val(arr[s], &mut freq, &mut freq_count, &mut count);
        }
        while e < qe {
            e += 1;
            add_val(arr[e], &mut freq, &mut freq_count, &mut count);
        }

        while s < qs {
            remove_val(arr[s], &mut freq, &mut freq_count, &mut count);
            s += 1;
        }
        while e > qe {
            remove_val(arr[e], &mut freq, &mut freq_count, &mut count);
            e -= 1;
        }

        ans[query[i].idx] = count;
    }

    for i in 0..m {
        writeln!(out, "{}", ans[i]).unwrap();
    }
}

fn add_val(val: usize, freq: &mut [usize], freq_count: &mut [usize], cur_max: &mut usize) {
    let old = freq[val];
    if old > 0 {
        freq_count[old] -= 1;
    }
    freq[val] += 1;
    let newf = old + 1;
    freq_count[newf] += 1;
    if newf > *cur_max {
        *cur_max = newf;
    }
}

fn remove_val(val: usize, freq: &mut [usize], freq_count: &mut [usize], cur_max: &mut usize) {
    let old = freq[val];
    if old == 0 {
        return;
    }
    freq_count[old] -= 1;
    freq[val] -= 1;
    let newf = old - 1;
    if newf > 0 {
        freq_count[newf] += 1;
    }
    while *cur_max > 0 && freq_count[*cur_max] == 0 {
        *cur_max -= 1;
    }
}
