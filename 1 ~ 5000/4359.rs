use io::Write;
use std::{
    collections::{HashMap, HashSet},
    io, str,
};

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

    let (t, p) = (scan.token::<usize>(), scan.token::<usize>());

    let mut map: HashMap<i64, Vec<bool>> = HashMap::new();

    for i in 1..=t {
        map.entry(i as i64).or_insert(vec![false; p + 1]);
    }

    while let Some(line) = scan.line_eof() {
        let mut parts = line.split_whitespace();
        let (Some(human), Some(tree)) = (parts.next(), parts.next()) else {
            continue;
        };

        let human = human.parse::<i64>().unwrap();
        let tree = tree.parse::<usize>().unwrap();

        let entry = map.entry(human).or_insert(vec![false; p + 1]);
        entry[tree] = true;
    }

    let mut set = HashSet::new();
    for trees in map.values_mut() {
        set.insert(format!("{:?}", trees));
    }
    write!(out, "{}", set.len());
}
