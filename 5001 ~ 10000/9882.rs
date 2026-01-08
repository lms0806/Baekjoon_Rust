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
            buf_str: Vec::with_capacity(1 << 16),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    #[inline(always)]
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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok()?);
            }
            self.buf_str.clear();
            let n = self.reader.read_until(b'\n', &mut self.buf_str).ok()?;
            if n == 0 {
                return None;
            }
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
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let mut arr = (0..12).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

    arr.sort_unstable_by(|a, b| b.cmp(a));

    let mut answer = i64::MAX;

    dfs(
        0,
        0,
        0,
        0,
        i64::MAX,
        i64::MIN,
        &mut answer,
        &mut vec![false; 12],
        &arr,
    );

    write!(out, "{}", answer).unwrap();
}

fn dfs(
    sum: i64,
    idx: usize,
    count: i64,
    team: i64,
    min: i64,
    max: i64,
    answer: &mut i64,
    visited: &mut [bool],
    arr: &Vec<i64>,
) {
    if max - min >= *answer {
        return;
    }

    if team == 4 {
        *answer = (*answer).min(max - min);
        return;
    }

    if count == 3 {
        dfs(
            0,
            0,
            0,
            team + 1,
            min.min(sum),
            max.max(sum),
            answer,
            visited,
            arr,
        );
        return;
    }

    for i in idx..12 {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        dfs(
            sum + arr[i],
            i + 1,
            count + 1,
            team,
            min,
            max,
            answer,
            visited,
            arr,
        );
        visited[i] = false;
    }
}
