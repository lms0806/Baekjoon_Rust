use io::Write;
use std::collections::BTreeMap;
use std::{collections::HashMap, io, str};

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
        input.trim().to_string()
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).ok()?;
        if bytes_read == 0 {
            return None; // EOF
        }
        Some(input.trim().to_string())
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let (mut v, mut sum) = (vec![0; n], vec![0; n]);

    let mut map: HashMap<i64, Vec<usize>> = HashMap::new();
    for i in 0..n {
        v[i] = scan.token::<i64>();

        if !map.contains_key(&v[i]) {
            map.insert(v[i], vec![]);
        }

        map.get_mut(&v[i]).unwrap().push(i);

        sum[i] = v[i];
        if i > 0 {
            sum[i] += sum[i - 1];
        }
    }

    let mut result: BTreeMap<i64, i64> = BTreeMap::new();
    for (key, value) in map.iter() {
        if value.len() < 2 {
            if let Some(cnt) = result.get_mut(&key) {
                *cnt += 1;
            } else {
                result.insert(*key, 1);
            }
            continue;
        }

        let num = sum[value[value.len() - 1]] - sum[value[0]] + v[value[0]];

        if let Some(cnt) = result.get_mut(&num) {
            *cnt += 1;
        } else {
            result.insert(num, 1);
        }
    }

    if let Some((key, value)) = result.iter().rev().next() {
        write!(out, "{} {}", key, value).unwrap();
    }
}
