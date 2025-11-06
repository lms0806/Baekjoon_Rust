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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok();
            }
            self.buf_str.clear();
            if self.reader.read_until(b'\n', &mut self.buf_str).unwrap() == 0 {
                return None;
            }
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

    loop {
        let s = scan.line();

        if s.trim() == "#" {
            break;
        }

        let arr = s.split_whitespace().collect::<Vec<_>>();
        let mut visited = vec![false; 21];

        let mut now = arr[0].parse::<usize>().unwrap();
        visited[now] = true;

        let mut check = true;
        for i in 1..arr.len() {
            let (ud, num) = arr[i].split_at(1);

            if ud == "U" {
                now += num.parse::<usize>().unwrap();
            } else {
                now -= num.parse::<usize>().unwrap();
            }

            if now > 20 || now < 1 || visited[now] {
                check = false;
                break;
            }
            visited[now] = true;
        }

        if check {
            let mut unvisited = Vec::new();
            for i in 1..21 {
                if !visited[i] {
                    unvisited.push(i.to_string())
                }
            }

            if unvisited.is_empty() {
                writeln!(out, "none").unwrap();
            } else {
                writeln!(out, "{}", unvisited.join(" ")).unwrap();
            }
        } else {
            writeln!(out, "illegal").unwrap();
        }
    }
}
