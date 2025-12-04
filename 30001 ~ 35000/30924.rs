use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
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

    let mut arr: Vec<i32> = (1..=10000).collect();

    shuffle(&mut arr);

    let mut a = 0;
    for &val in &arr {
        writeln!(out, "? A {}", val).unwrap();
        out.flush().unwrap();

        if scan.token::<usize>() == 1 {
            a = val;
            break;
        }
    }

    shuffle(&mut arr);

    let mut b = 0;
    for &val in &arr {
        writeln!(out, "? B {}", val).unwrap();
        out.flush().unwrap();

        if scan.token::<usize>() == 1 {
            b = val;
            break;
        }
    }

    writeln!(out, "! {}", a + b).unwrap();
    out.flush().unwrap();
}

fn shuffle(arr: &mut [i32]) {
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let len = arr.len();
    for i in (1..len).rev() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (seed % (i as u64 + 1)) as usize;
        arr.swap(i, j);
    }
}
