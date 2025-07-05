use io::Write;
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

struct Node {
    x: i64,
    y: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, h) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let mut v: Vec<Node> = Vec::new();
    let mut arr = vec![vec![0; n]; n];
    let mut start = Node { x: 0, y: 0 };
    for i in 0..n {
        for j in 0..n {
            arr[i][j] = scan.token::<i64>();

            if arr[i][j] == 1 {
                start = Node {
                    x: i as i64,
                    y: j as i64,
                };
            } else if arr[i][j] == 2 {
                v.push(Node {
                    x: i as i64,
                    y: j as i64,
                });
            }
        }
    }

    let mut visited = vec![false; v.len()];

    let mut answer = 0;
    dfs(&start, m, 0, &start, &mut visited, &v, h, &mut answer);
    write!(out, "{}", answer).unwrap();
}

fn dfs(
    start: &Node,
    m: i64,
    count: i64,
    now: &Node,
    visited: &mut Vec<bool>,
    v: &Vec<Node>,
    h: i64,
    answer: &mut i64,
) {
    if m >= dist(start, now) {
        *answer = (*answer).max(count);
    }

    for i in 0..v.len() {
        if !visited[i] {
            let dif = dist(now, &v[i]);

            if m >= dif {
                visited[i] = true;
                dfs(start, m - dif + h, count + 1, &v[i], visited, v, h, answer);
                visited[i] = false;
            }
        }
    }
}

fn dist(a: &Node, b: &Node) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
