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

    let n = scan.token::<usize>();
    let arr: Vec<Vec<i64>> = (0..n)
        .map(|_| (0..n).map(|_| scan.token::<i64>()).collect())
        .collect();
    let mut dp = vec![vec![-1; n]; n];

    let (dx, dy) = (vec![1, 0, -1, 0], vec![0, 1, 0, -1]);
    let mut answer = 0;
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] == -1 {
                answer = answer.max(dfs(i, j, n, &arr, &mut dp, &dx, &dy));
            }
        }
    }
    write!(out, "{}", answer).unwrap();
}

fn dfs(
    x: usize,
    y: usize,
    n: usize,
    arr: &Vec<Vec<i64>>,
    dp: &mut Vec<Vec<i64>>,
    dx: &Vec<i64>,
    dy: &Vec<i64>,
) -> i64 {
    if dp[x][y] != -1 {
        return dp[x][y];
    }
    dp[x][y] = 1;

    for i in 0..4 {
        let (nx, ny) = (x as i64 + dx[i], y as i64 + dy[i]);

        if nx >= 0 && nx < n as i64 && ny >= 0 && ny < n as i64 {
            let (nx, ny) = (nx as usize, ny as usize);

            if arr[x][y] < arr[nx][ny] {
                dp[x][y] = dp[x][y].max(
                    if dp[nx][ny] == -1 {
                        dfs(nx, ny, n, arr, dp, dx, dy)
                    } else {
                        dp[nx][ny]
                    } + 1,
                );
            }
        }
    }
    dp[x][y]
}
