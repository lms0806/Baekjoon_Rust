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

    let mut box_map = [[0usize; 9]; 9];
    for r in 0..9 {
        for c in 0..9 {
            box_map[r][c] = (r / 3) * 3 + (c / 3);
        }
    }

    let mut arr = [[0; 9]; 9];

    let mut total_used = 0;
    let (mut row, mut col, mut bx) = ([[false; 10]; 9], [[false; 10]; 9], [[false; 10]; 9]);
    for i in 0..9 {
        for j in 0..9 {
            arr[i][j] = scan.token::<usize>();

            if arr[i][j] != 0 {
                total_used += 1;
                row[i][arr[i][j]] = true;
                col[j][arr[i][j]] = true;
                bx[box_map[i][j]][arr[i][j]] = true;
            }
        }
    }

    solve(&mut arr, total_used, &mut row, &mut col, &mut bx, &box_map);

    for i in 0..9 {
        for j in 0..9 {
            write!(out, "{} ", arr[i][j]).unwrap();
        }
        writeln!(out).unwrap();
    }
}

fn solve(
    arr: &mut [[usize; 9]; 9],
    total_used: usize,
    row: &mut [[bool; 10]; 9],
    col: &mut [[bool; 10]; 9],
    bx: &mut [[bool; 10]; 9],
    box_map: &[[usize; 9]; 9],
) -> bool {
    if total_used == 81 {
        return true;
    }

    // box
    for i in 0..9 {
        for num in 1..=9 {
            if bx[i][num] {
                continue;
            }

            let (mut target_r, mut target_c) = (0, 0);
            let mut count = 0;

            // 해당 박스의 시작 좌표 계산
            let start_r = (i / 3) * 3;
            let start_c = (i % 3) * 3;

            // 박스 내부 순회
            'goto: for r_off in 0..3 {
                for c_off in 0..3 {
                    let r = start_r + r_off;
                    let c = start_c + c_off;

                    if arr[r][c] == 0 && !row[r][num] && !col[c][num] {
                        target_r = r;
                        target_c = c;
                        count += 1;
                        if count > 1 {
                            break 'goto;
                        }
                    }
                }
            }

            if count == 0 {
                return false;
            }

            if count > 1 {
                continue;
            }

            let r = target_r;
            let c = target_c;

            arr[r][c] = num;
            row[r][num] = true;
            col[c][num] = true;
            bx[i][num] = true;

            if solve(arr, total_used + 1, row, col, bx, box_map) {
                return true;
            }

            arr[r][c] = 0;
            row[r][num] = false;
            col[c][num] = false;
            bx[i][num] = false;
            return false;
        }
    }

    // row
    for i in 0..9 {
        for num in 1..=9 {
            if row[i][num] {
                continue;
            }

            let (mut jdx, mut count) = (0, 0);
            for j in 0..9 {
                let b = box_map[i][j];
                if arr[i][j] == 0 && !col[j][num] && !bx[b][num] {
                    jdx = j;
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }

            if count == 0 {
                return false;
            }

            if count > 1 {
                continue;
            }

            let j = jdx;
            let b = box_map[i][j];

            arr[i][j] = num;
            row[i][num] = true;
            col[j][num] = true;
            bx[b][num] = true;

            if solve(arr, total_used + 1, row, col, bx, box_map) {
                return true;
            }

            arr[i][j] = 0;
            row[i][num] = false;
            col[j][num] = false;
            bx[b][num] = false;
            return false;
        }
    }

    // col
    for i in 0..9 {
        for num in 1..=9 {
            if col[i][num] {
                continue;
            }

            let (mut jdx, mut count) = (0, 0);
            for j in 0..9 {
                let b = box_map[j][i];
                if arr[j][i] == 0 && !row[j][num] && !bx[b][num] {
                    jdx = j;
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }

            if count == 0 {
                return false;
            }

            if count > 1 {
                continue;
            }

            let j = jdx;
            let b = box_map[j][i];

            arr[j][i] = num;
            row[j][num] = true;
            col[i][num] = true;
            bx[b][num] = true;

            if solve(arr, total_used + 1, row, col, bx, box_map) {
                return true;
            }

            arr[j][i] = 0;
            row[j][num] = false;
            col[i][num] = false;
            bx[b][num] = false;
            return false;
        }
    }

    for i in 0..9 {
        for j in 0..9 {
            if arr[i][j] != 0 {
                continue;
            }

            let b = box_map[i][j];

            for num in 1..=9 {
                if row[i][num] || col[j][num] || bx[b][num] {
                    continue;
                }

                arr[i][j] = num;
                row[i][num] = true;
                col[j][num] = true;
                bx[b][num] = true;

                if solve(arr, total_used + 1, row, col, bx, box_map) {
                    return true;
                }

                arr[i][j] = 0;
                row[i][num] = false;
                col[j][num] = false;
                bx[b][num] = false;
            }
            return false;
        }
    }

    false
}
