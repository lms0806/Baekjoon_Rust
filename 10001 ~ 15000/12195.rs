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

    for i in 1..=scan.token::<usize>() {
        let mut arr = [0; 10];
        for i in 0..10 {
            arr[i] = scan.token::<i64>();
        }
        let target = scan.token::<usize>();

        let mut cost = vec![-1i64; target + 1];
        for i in 0..=target {
            cost[i] = check(i, &arr);
        }

        let mut answer = if cost[target] == -1 {
            i64::MAX
        } else {
            cost[target] + 1
        };

        for i in 2..=target {
            let result = cost[i];
            if result == -1 || result + 1 >= answer {
                continue;
            }
            dfs(i as i64, result, &arr, target as i64, &cost, &mut answer);
        }

        if answer == i64::MAX {
            writeln!(out, "Case #{}: Impossible", i).unwrap();
        } else {
            writeln!(out, "Case #{}: {}", i, answer).unwrap();
        }
    }
}

fn dfs(sum: i64, count: i64, arr: &[i64; 10], target: i64, cost: &[i64], answer: &mut i64) {
    if count >= *answer || sum > target {
        return;
    }

    if sum == target {
        *answer = (*answer).min(count + 1);
        return;
    }

    for i in 2..=target / sum {
        if sum * i > target {
            break;
        }

        let result = cost[i as usize];
        if result != -1 {
            dfs(sum * i, count + result + 1, arr, target, cost, answer);
        }
    }
}

fn check(mut num: usize, arr: &[i64; 10]) -> i64 {
    if num == 0 {
        return if arr[0] == 1 { 1 } else { -1 };
    }

    let mut result = 0;
    while num > 0 {
        if arr[num % 10] == 0 {
            return -1;
        }

        result += 1;
        num /= 10;
    }
    result
}
