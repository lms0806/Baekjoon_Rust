use std::collections::{BTreeMap, HashSet};
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
        self.buf_iter = "".split_ascii_whitespace();
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let mut check = true;

    loop {
        if scan.token_eof::<i32>().is_none() {
            break;
        }

        let len = scan.token::<usize>();
        let mut set = HashSet::new();
        for _ in 0..len {
            set.insert(scan.token::<i32>());
        }

        let mut map = BTreeMap::new();
        let (mut num, mut count) = (1, 0);

        if !check {
            writeln!(out).unwrap();
        }

        check = false;

        loop {
            match scan.token::<String>().as_bytes()[0] {
                b'a' => {
                    *map.entry(scan.token::<i64>()).or_insert(0) += 1;
                }
                b'p' => {
                    num = scan.token::<i64>();
                }
                b'r' => {
                    count += 1;

                    if map.is_empty() {
                        if set.contains(&count) {
                            writeln!(out, "-1").unwrap();
                        }
                    } else {
                        let key = if num == 1 {
                            *map.iter().next().unwrap().0
                        } else {
                            *map.iter().next_back().unwrap().0
                        };

                        if let Some(cnt) = map.get_mut(&key) {
                            *cnt -= 1;
                            if *cnt == 0 {
                                map.remove(&key);
                            }
                        }

                        if set.contains(&count) {
                            writeln!(out, "{}", key).unwrap();
                        }
                    }
                }
                _ => break,
            };
        }
    }
}
