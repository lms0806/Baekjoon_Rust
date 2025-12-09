use std::collections::{BTreeSet, HashMap, VecDeque};
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

    let mut map = HashMap::new();
    let mut answer = HashMap::new();
    let mut set = BTreeSet::new();
    for _ in 0..n {
        let (a, _, b) = (
            scan.token::<char>(),
            scan.token::<String>(),
            scan.token::<char>(),
        );

        set.insert(a);

        if b.is_ascii_lowercase() {
            answer.entry(a).or_insert_with(BTreeSet::new).insert(b);
        } else {
            map.entry(b).or_insert_with(Vec::new).push(a);
            set.insert(b);
        }
    }

    let mut queue = VecDeque::new();
    for &s in &set {
        queue.push_back(s);
    }

    while let Some(x) = queue.pop_front() {
        if let Some(next) = map.get(&x) {
            let from = answer.entry(x).or_default().clone();
            for &y in next {
                let to = answer.entry(y).or_default();

                let before = to.len();
                to.extend(&from);

                if to.len() != before {
                    queue.push_back(y);
                }
            }
        }
    }

    for s in set {
        let num = answer.get(&s).cloned().unwrap_or_else(BTreeSet::new);

        write!(out, "{} = {{", s).unwrap();
        for (i, e) in num.iter().enumerate() {
            if i > 0 {
                write!(out, ",").unwrap();
            }
            write!(out, "{}", e).unwrap();
        }
        writeln!(out, "}}").unwrap();
    }
}
