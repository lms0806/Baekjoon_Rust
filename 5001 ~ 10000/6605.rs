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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut ordinal = vec!["th"; 100];
    for i in (1..100).step_by(10) {
        ordinal[i] = "st";
    }
    for i in (2..100).step_by(10) {
        ordinal[i] = "nd";
    }
    for i in (3..100).step_by(10) {
        ordinal[i] = "rd";
    }
    ordinal[11] = "th";
    ordinal[12] = "th";
    ordinal[13] = "th";

    let mut arr = Vec::new();

    let mut v2 = 1i64;
    while v2 < 2_000_000_001 {
        let mut v3 = v2;
        while v3 < 2_000_000_001 {
            let mut v5 = v3;
            while v5 < 2_000_000_001 {
                let mut v7 = v5;
                while v7 < 2_000_000_001 {
                    arr.push(v7);
                    v7 *= 7;
                }
                v5 *= 5;
            }
            v3 *= 3;
        }
        v2 *= 2;
    }

    arr.sort_unstable();

    loop {
        let n = scan.token::<usize>();
        if n == 0 {
            break;
        }

        writeln!(
            out,
            "The {}{} humble number is {}.",
            n,
            ordinal[n % 100],
            arr[n - 1]
        )
        .unwrap();
    }
}
