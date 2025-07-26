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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut arr = (0..5)
        .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut select = vec![false; 12];
    let mut vec = vec![];

    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if 'A' <= arr[i][j] && arr[i][j] <= 'L' {
                select[(arr[i][j] as u8 - b'A') as usize] = true;
            } else if arr[i][j] == 'x' {
                vec.push((i, j));
            }
        }
    }

    let which = [
        [(1, 1), (1, 3), (1, 5), (1, 7)],
        [(3, 1), (2, 2), (1, 3), (0, 4)],
        [(1, 1), (2, 2), (3, 3), (4, 4)],
        [(3, 1), (3, 3), (3, 5), (3, 7)],
        [(4, 4), (3, 5), (2, 6), (1, 7)],
        [(0, 4), (1, 5), (2, 6), (3, 7)],
    ];

    dfs(0, &mut arr, &mut select, &vec, &which);

    arr.iter().for_each(|value| {
        writeln!(out, "{}", value.iter().collect::<String>()).unwrap();
    });
}

fn dfs(
    idx: usize,
    arr: &mut Vec<Vec<char>>,
    select: &mut Vec<bool>,
    vec: &Vec<(usize, usize)>,
    which: &[[(usize, usize); 4]; 6],
) -> bool {
    if idx == vec.len() {
        if check(arr, which) {
            return true;
        }
        return false;
    }

    let (x, y) = vec[idx];
    for i in 0..12 {
        if select[i] {
            continue;
        }

        select[i] = true;
        arr[x][y] = (i as u8 + b'A') as char;
        if dfs(idx + 1, arr, select, vec, which) {
            return true;
        };
        arr[x][y] = 'x';
        select[i] = false;
    }
    false
}

fn check(arr: &Vec<Vec<char>>, which: &[[(usize, usize); 4]; 6]) -> bool {
    which.iter().all(|line| {
        line.iter()
            .map(|&(x, y)| (arr[x][y] as u8 - b'A' + 1) as i64)
            .sum::<i64>()
            == 26
    })
}
