use io::Write;
use std::cmp::Ordering;
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

    let n = scan.token::<usize>();
    let answer = (0..n).map(|_| scan.token::<char>()).collect::<Vec<_>>();

    let mut arr = vec![];
    for _ in 0..scan.token::<usize>() {
        let id = scan.token::<i64>();
        let mut count = 0;
        for i in 0..n {
            if answer[i] == scan.token::<char>() {
                count += 1;
            }
        }
        arr.push((id, count));
    }

    let sort = scan.token::<String>();
    arr.sort_by(|a, b| {
        let primary_ordering = match sort.as_str() {
            "STUDENT_ID_ASC" => a.0.cmp(&b.0),
            "STUDENT_ID_DESC" => b.0.cmp(&a.0),
            "GRADE_ASC" => a.1.cmp(&b.1),
            "GRADE_DESC" => b.1.cmp(&a.1),
            _ => Ordering::Equal,
        };

        primary_ordering.then(a.0.cmp(&b.0))
    });

    for (id, grade) in arr {
        writeln!(out, "{} {}", id, grade).unwrap();
    }
}
