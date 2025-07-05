use io::Write;
use std::{collections::VecDeque, io, str};

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

    let mut deque: VecDeque<char> = VecDeque::new();
    let mut ball = 0;
    let mut wall = 0;
    let mut state = 0; // 0 : 뒤앞(가로), 1 : 뒤앞(세로 - 앞으로 떨어짐), 2 : 앞뒤, 3 : 앞뒤(세로 - 뒤로 떨어짐)

    for _ in 0..n {
        let cmd = scan.token::<String>();

        if cmd == "pop" {
            if deque.is_empty() {
                continue;
            }

            if deque.pop_back().unwrap() == 'b' {
                ball -= 1;
            } else {
                wall -= 1;
            }
        } else {
            let value = scan.token::<char>();

            if cmd == "push" {
                if value == 'b' {
                    ball += 1;
                } else {
                    wall += 1;
                }

                deque.push_front(value);
            } else if cmd == "rotate" {
                if value == 'r' {
                    state = (state + 1) % 4;
                } else {
                    state = (state + 3) % 4;
                }
            } else {
                writeln!(out, "{}", if value == 'b' { ball } else { wall }).unwrap();
            }
        }

        if state == 1 {
            while !deque.is_empty() {
                if deque.back().unwrap() == &'w' {
                    break;
                }
                ball -= 1;
                deque.pop_back();
            }
        } else if state == 3 {
            while !deque.is_empty() {
                if deque.front().unwrap() == &'w' {
                    break;
                }
                ball -= 1;
                deque.pop_front();
            }
        }
    }
}
