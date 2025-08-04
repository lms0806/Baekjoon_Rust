use io::Write;
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

    let t = scan.token::<usize>();

    for _ in 0..t {
        scan.token::<i64>();

        writeln!(
            out,
            "{}",
            solve(scan.token::<String>().chars().collect::<Vec<char>>())
        )
        .unwrap();
    }
}

fn solve(arr: Vec<char>) -> &'static str {
    let mut idx = None;

    for i in (1..arr.len()).step_by(2) {
        if arr[i] == '-' {
            idx = Some(i);
            break;
        }
    }

    match idx {
        None => "YES",
        Some(idx) => {
            for i in (idx + 2..arr.len()).step_by(2) {
                let num_char = arr[i + 1];

                if num_char != '0' {
                    return "NO";
                }
            }

            "YES"
        }
    }
}
