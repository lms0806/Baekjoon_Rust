use io::Write;
use std::collections::{BTreeSet, HashMap};
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

    let q = scan.token::<usize>();

    let (mut find_map, mut take_map): (HashMap<String, BTreeSet<String>>, HashMap<String, String>) =
        (HashMap::new(), HashMap::new());
    for _ in 0..q {
        let op = scan.token::<String>();

        match op.as_str() {
            "PUT" => {
                let (a, b) = (scan.token::<String>(), scan.token::<String>());
                find_map
                    .entry(a.clone())
                    .or_insert(BTreeSet::new())
                    .insert(b.clone());
                take_map.insert(b, a);
            }
            "FIND" => {
                let a = scan.token::<String>();
                if let Some(set) = find_map.get(&a) {
                    if set.is_empty() {
                        writeln!(out, "NOT FOUND").unwrap();
                    } else {
                        for val in set {
                            write!(out, "{} ", val).unwrap();
                        }
                        writeln!(out).unwrap();
                    }
                } else {
                    writeln!(out, "NOT FOUND").unwrap();
                }
            }
            "TAKE" => {
                let a = scan.token::<String>();
                if let Some(b) = take_map.get(&a) {
                    if let Some(set) = find_map.get_mut(b) {
                        set.remove(&a);

                        if set.is_empty() {
                            find_map.remove(b);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
