use std::collections::HashMap;
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

    let n = scan.token::<usize>();

    let mut idx = 0;
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut parent = (0..n * 2).collect::<Vec<_>>();
    for _ in 0..n {
        let (a, _, b) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );

        if !map.contains_key(&a) {
            map.insert(a.clone(), idx);
            idx += 1;
        }

        if !map.contains_key(&b) {
            map.insert(b.clone(), idx);
            idx += 1;
        }

        union(&mut parent, *map.get(&a).unwrap(), *map.get(&b).unwrap());
    }

    for _ in 0..scan.token::<usize>() {
        if let (Some(&id_a), Some(&id_b)) = (
            map.get(&scan.token::<String>()),
            map.get(&scan.token::<String>()),
        ) {
            if find(&mut parent, id_a) == find(&mut parent, id_b) {
                writeln!(out, "Related").unwrap();
            } else {
                writeln!(out, "Not Related").unwrap();
            }
        } else {
            writeln!(out, "Not Related").unwrap();
        }
    }
}

fn find(parent: &mut Vec<usize>, mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

fn union(parent: &mut Vec<usize>, i: usize, j: usize) {
    let root_i = find(parent, i);
    let root_j = find(parent, j);
    if root_i != root_j {
        parent[root_i] = root_j;
    }
}
