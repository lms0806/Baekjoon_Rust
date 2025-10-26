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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).expect("Failed read");

        if bytes_read == 0 {
            None
        } else {
            Some(input)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let arr = (0..scan.token::<usize>())
        .map(|_| (scan.token::<String>(), scan.token::<i64>()))
        .collect::<Vec<_>>();

    let mut answer = vec![];
    dfs(0, 0, &mut vec![], &arr, &mut answer);

    answer.sort();

    writeln!(out, "{}", answer.len()).unwrap();
    for i in 0..answer.len() {
        writeln!(out, "{}", answer[i]).unwrap();
    }
}

fn dfs(
    idx: usize,
    sum: i64,
    path: &mut Vec<String>,
    arr: &Vec<(String, i64)>,
    answer: &mut Vec<String>,
) {
    if idx == arr.len() {
        if sum >= 76 {
            answer.push(path.join("-"));
        }
        return;
    }

    path.push(arr[idx].0.clone());
    dfs(idx + 1, sum + arr[idx].1, path, arr, answer);
    path.pop();
    dfs(idx + 1, sum, path, arr, answer);
}
