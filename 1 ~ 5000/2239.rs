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
            buf_str: Vec::with_capacity(1 << 16),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    #[inline(always)]
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

    pub fn token_eof<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok()?);
            }
            self.buf_str.clear();
            let n = self.reader.read_until(b'\n', &mut self.buf_str).ok()?;
            if n == 0 {
                return None;
            }
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
    let reader = io::BufReader::with_capacity(1 << 20, stdin.lock());
    let mut scan = UnsafeScanner::new(reader);
    let mut out = io::BufWriter::new(stdout.lock());

    let mut arr = vec![vec![0; 9]; 9];

    for i in 0..9 {
        let line = scan.line();
        for (j, ch) in line.chars().enumerate() {
            arr[i][j] = (ch as u8 - b'0') as usize;
        }
    }

    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            nine_check(&mut arr, i, j);
        }
    }

    for i in 0..9 {
        row_check(&mut arr, i);
    }

    for i in 0..9 {
        col_check(&mut arr, i);
    }

    solve(&mut arr);

    for i in 0..9 {
        for j in 0..9 {
            write!(out, "{}", arr[i][j]).unwrap();
        }
        writeln!(out).unwrap();
    }
}

fn solve(arr: &mut Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if arr[i][j] != 0 {
                continue;
            }

            for num in 1..=9 {
                if is_valid(arr, i, j, num) {
                    arr[i][j] = num;
                    if solve(arr) {
                        return true;
                    }
                    arr[i][j] = 0;
                }
            }
            return false;
        }
    }
    true
}

fn is_valid(arr: &Vec<Vec<usize>>, x: usize, y: usize, num: usize) -> bool {
    for j in 0..9 {
        if arr[x][j] == num {
            return false;
        }
    }

    for i in 0..9 {
        if arr[i][y] == num {
            return false;
        }
    }

    let (sx, sy) = ((x / 3) * 3, (y / 3) * 3);
    for i in sx..sx + 3 {
        for j in sy..sy + 3 {
            if arr[i][j] == num {
                return false;
            }
        }
    }

    true
}

fn nine_check(arr: &mut Vec<Vec<usize>>, a: usize, b: usize) {
    let (mut idx, mut jdx) = (10, 10);
    let mut check = vec![false; 10];
    for i in a..a + 3 {
        for j in b..b + 3 {
            check[arr[i][j]] = true;

            if arr[i][j] == 0 {
                if idx != 10 {
                    return;
                }
                idx = i;
                jdx = j;
            }
        }
    }

    let mut num = 0;
    for i in 1..10 {
        if !check[i] {
            if num != 0 {
                return;
            }
            num = i;
        }
    }

    arr[idx][jdx] = num;
}

fn col_check(arr: &mut Vec<Vec<usize>>, b: usize) {
    let mut idx = 10;
    let mut check = vec![false; 10];
    for i in 0..9 {
        check[arr[i][b]] = true;

        if arr[i][b] == 0 {
            if idx != 10 {
                return;
            }
            idx = i;
        }
    }

    if idx == 10 {
        return;
    }

    let mut num = 0;
    for i in 1..10 {
        if !check[i] {
            if num != 0 {
                return;
            }
            num = i;
        }
    }

    arr[idx][b] = num;
}

fn row_check(arr: &mut Vec<Vec<usize>>, a: usize) {
    let mut idx = 10;
    let mut check = vec![false; 10];
    for i in 0..9 {
        check[arr[a][i]] = true;

        if arr[a][i] == 0 {
            if idx != 10 {
                return;
            }
            idx = i;
        }
    }

    if idx == 10 {
        return;
    }

    let mut num = 0;
    for i in 1..10 {
        if !check[i] {
            if num != 0 {
                return;
            }
            num = i;
        }
    }

    arr[a][idx] = num;
}
