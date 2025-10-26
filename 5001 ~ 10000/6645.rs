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

    loop {
        let (n, s) = (scan.token::<usize>(), scan.token::<String>());

        if n == 0 {
            break;
        }

        let mut arr = vec![];
        for _ in 0..n {
            arr.push((
                scan.token::<String>(),
                scan.token::<String>(),
                scan.token::<f64>(),
            ));
        }

        writeln!(out, "{}", s).unwrap();
        for i in 0..n {
            let mut result = vec![];
            for j in 0..n {
                if i == j || arr[i].1 == arr[j].1 {
                    continue;
                }

                if (arr[i].1 == "buy" && arr[i].2 >= arr[j].2)
                    || (arr[i].1 == "sell" && arr[i].2 <= arr[j].2)
                {
                    result.push(arr[j].0.as_str());
                }
            }

            if result.is_empty() {
                result.push("NO-ONE");
            }
            writeln!(out, "{}: {}", arr[i].0, result.join(" ")).unwrap();
        }
    }
}
