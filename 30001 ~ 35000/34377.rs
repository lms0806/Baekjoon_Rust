use std::collections::{HashMap, HashSet, VecDeque};
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let s = scan.token::<String>();

    let mut map = HashMap::new();
    for _ in 0..n {
        map.entry(scan.token::<String>())
            .or_insert_with(Vec::new)
            .push(scan.token::<String>());
    }

    if !map.contains_key(&s) {
        write!(out, "NO BLACK HOLE").unwrap();
        return;
    }

    let mut queue = VecDeque::new();
    let mut set = HashSet::new();
    for value in map.get(&s).unwrap() {
        queue.push_back((value, 1));
        set.insert(value);
    }

    let mut answer = i64::MAX;
    while let Some((value, cnt)) = queue.pop_front() {
        if value == &s {
            answer = answer.min(cnt);
            continue;
        }

        if !map.contains_key(value) {
            continue;
        }

        for v in map.get(value).unwrap() {
            if set.contains(v) {
                continue;
            }
            queue.push_back((v, cnt + 1));
            set.insert(v);
        }
    }

    if answer == i64::MAX {
        write!(out, "NO BLACK HOLE").unwrap();
    } else {
        write!(out, "{}", answer).unwrap();
    }
}
