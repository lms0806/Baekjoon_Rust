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
            };
        }
    }

    pub fn line(&mut self) -> String {
        self.buf_str.clear();
        self.reader.read_until(b'\n', &mut self.buf_str).unwrap();
        let s = std::str::from_utf8(&self.buf_str).unwrap();
        s.trim_end().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let patterns: Vec<Vec<&str>> = vec![
        vec![
            "..#####..",
            ".##...##.",
            "##.....##",
            "##.....##",
            "##.....##",
            ".##...##.",
            "..#####..",
            ".........",
        ],
        vec![
            "....##...",
            "..####...",
            "....##...",
            "....##...",
            "....##...",
            "....##...",
            "..######.",
            ".........",
        ],
        vec![
            ".#######.",
            "##.....##",
            ".......##",
            ".#######.",
            "##.......",
            "##.......",
            "#########",
            ".........",
        ],
        vec![
            ".#######.",
            "##.....##",
            ".......##",
            ".#######.",
            ".......##",
            "##.....##",
            ".#######.",
            ".........",
        ],
        vec![
            "##.......",
            "##....##.",
            "##....##.",
            "##....##.",
            "#########",
            "......##.",
            "......##.",
            ".........",
        ],
        vec![
            ".########",
            ".##......",
            ".##......",
            ".#######.",
            ".......##",
            ".##....##",
            "..######.",
            ".........",
        ],
        vec![
            ".#######.",
            "##.....##",
            "##.......",
            "########.",
            "##.....##",
            "##.....##",
            ".#######.",
            ".........",
        ],
        vec![
            ".########",
            ".##....##",
            ".....##..",
            "....##...",
            "...##....",
            "...##....",
            "...##....",
            ".........",
        ],
        vec![
            ".#######.",
            "##.....##",
            "##.....##",
            ".#######.",
            "##.....##",
            "##.....##",
            ".#######.",
            ".........",
        ],
        vec![
            ".#######.",
            "##.....##",
            "##.....##",
            ".########",
            ".......##",
            "##.....##",
            ".#######.",
            ".........",
        ],
    ];

    let n = scan.token::<usize>();

    for _ in 0..n {
        let input = (0..8).map(|_| scan.token::<String>()).collect::<Vec<_>>();

        for (num, pattern) in patterns.iter().enumerate() {
            let mut check = true;
            for i in 0..8 {
                if input[i] != pattern[i] {
                    check = false;
                    break;
                }
            }

            if check {
                write!(out, "{}", num).unwrap();
                break;
            }
        }
    }
}
