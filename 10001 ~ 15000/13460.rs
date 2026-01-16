use io::Write;
use std::collections::VecDeque;
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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut board = (0..n)
        .map(|_| scan.token::<String>().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut rx, mut ry, mut bx, mut by) = (0, 0, 0, 0);
    for i in 0..n {
        for j in 0..m {
            if board[i][j] == 'R' {
                rx = i as i64;
                ry = j as i64;
            } else if board[i][j] == 'B' {
                bx = i as i64;
                by = j as i64;
            }
        }
    }

    write!(out, "{}", bfs(rx, ry, bx, by, &mut board)).unwrap();
}

fn bfs(rx: i64, ry: i64, bx: i64, by: i64, board: &mut Vec<Vec<char>>) -> i64 {
    let mut queue = VecDeque::new();
    let mut visited =
        vec![vec![vec![vec![false; board[0].len()]; board.len()]; board[0].len()]; board.len()];
    visited[rx as usize][ry as usize][bx as usize][by as usize] = true;
    queue.push_back((rx, ry, bx, by, 0));

    let dist: [(i64, i64); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    while let Some((rx, ry, bx, by, count)) = queue.pop_front() {
        if count >= 10 {
            continue;
        }

        for i in 0..dist.len() {
            let (dx, dy) = dist[i];
            let (mut rnx, mut rny) = (rx, ry);
            let (mut bnx, mut bny) = (bx, by);

            let mut rgoal = false;
            let mut bgoal = false;
            loop {
                rnx += dx;
                rny += dy;

                if rnx < 0
                    || rnx >= board.len() as i64
                    || rny < 0
                    || rny >= board[0].len() as i64
                    || board[rnx as usize][rny as usize] == '#'
                {
                    rnx -= dx;
                    rny -= dy;
                    break;
                }

                if board[rnx as usize][rny as usize] == 'O' {
                    rgoal = true;
                    break;
                }
            }

            loop {
                bnx += dx;
                bny += dy;

                if bnx < 0
                    || bnx >= board.len() as i64
                    || bny < 0
                    || bny >= board[0].len() as i64
                    || board[bnx as usize][bny as usize] == '#'
                {
                    bnx -= dx;
                    bny -= dy;
                    break;
                }

                if board[bnx as usize][bny as usize] == 'O' {
                    bgoal = true;
                    break;
                }
            }

            if rgoal && !bgoal {
                return count + 1;
            }

            if !rgoal && !bgoal {
                if rnx == bnx && rny == bny {
                    (rnx, rny, bnx, bny) = reload(rx, ry, bx, by, rnx, rny, i);
                }
                if !visited[rnx as usize][rny as usize][bnx as usize][bny as usize] {
                    visited[rnx as usize][rny as usize][bnx as usize][bny as usize] = true;
                    queue.push_back((rnx, rny, bnx, bny, count + 1));
                }
            }
        }
    }
    -1
}

fn reload(rx: i64, ry: i64, bx: i64, by: i64, nx: i64, ny: i64, i: usize) -> (i64, i64, i64, i64) {
    let (mut rbx, mut rby, mut bbx, mut bby) = (0, 0, 0, 0);
    if i == 0 {
        rbx = nx;
        bbx = nx;
        if by > ry {
            rby = ny - 1;
            bby = ny;
        } else {
            rby = ny;
            bby = ny - 1;
        }
    } else if i == 1 {
        rbx = nx;
        bbx = nx;

        if by > ry {
            rby = ny;
            bby = ny + 1;
        } else {
            rby = ny + 1;
            bby = ny;
        }
    } else if i == 2 {
        rby = ny;
        bby = ny;
        if bx > rx {
            rbx = nx;
            bbx = nx + 1;
        } else {
            rbx = nx + 1;
            bbx = nx;
        }
    } else {
        rby = ny;
        bby = ny;
        if bx > rx {
            rbx = nx - 1;
            bbx = nx;
        } else {
            rbx = nx;
            bbx = nx - 1;
        }
    }
    (rbx, rby, bbx, bby)
}
