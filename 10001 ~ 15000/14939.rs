use io::Write;
use std::ptr::write;
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

    let mut arr = vec![vec![false; 10]; 10];
    for i in 0..10 {
        for (j, ch) in scan.token::<String>().chars().enumerate() {
            arr[i][j] = ch == 'O';
        }
    }

    let mut answer = i64::MAX;
    for k in 0..1024 {
        let mut count = 0;
        let mut vec = arr.clone();

        for i in 0..10 {
            if k & (1 << i) != 0 {
                change(0, i, &mut vec, [1, 0, -1, 0], [0, 1, 0, -1]);
                count += 1;
            }
        }

        for i in 1..10 {
            for j in 0..10 {
                if vec[i - 1][j] {
                    change(i, j, &mut vec, [1, 0, -1, 0], [0, 1, 0, -1]);
                    count += 1;
                }
            }
        }

        if vec.iter().flatten().all(|&b| !b) {
            answer = answer.min(count);
        }
    }
    write!(out, "{}", answer).unwrap();
}

fn change(x: usize, y: usize, arr: &mut Vec<Vec<bool>>, dx: [i32; 4], dy: [i32; 4]) {
    arr[x][y] = !arr[x][y];

    for i in 0..4 {
        let (nx, ny) = (x as i32 + dx[i], y as i32 + dy[i]);

        if nx < 0 || nx >= 10 || ny < 0 || ny >= 10 {
            continue;
        }

        arr[nx as usize][ny as usize] = !arr[nx as usize][ny as usize];
    }
}
