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

#[derive(Debug, Clone)]
struct Node {
    x: i64,
    y: i64,
    count: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

    let board: Vec<Vec<char>> = (0..n).map(|_| scan.line().chars().collect()).collect();

    let (mut jinwoo, mut seungchan) = (
        Node {
            x: 0,
            y: 0,
            count: 0,
        },
        Node {
            x: 0,
            y: 0,
            count: 0,
        },
    );
    let mut teacher = Vec::new();

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'S' {
                seungchan = Node {
                    x: i as i64,
                    y: j as i64,
                    count: 0,
                };
            } else if board[i][j] == 'J' {
                jinwoo = Node {
                    x: i as i64,
                    y: j as i64,
                    count: 0,
                };
            } else if board[i][j] == 'T' {
                teacher.push(Node {
                    x: i as i64,
                    y: j as i64,
                    count: 0,
                })
            }
        }
    }

    let (mut visited_j, mut visited_s) = (vec![vec![i64::MAX; m]; n], vec![vec![i64::MAX; m]; n]);

    bfs(n, m, &jinwoo, &mut visited_j, &board);
    bfs(n, m, &seungchan, &mut visited_s, &board);

    let mut answer = visited_j[seungchan.x as usize][seungchan.y as usize].min(i64::MAX);

    for i in 0..teacher.len() {
        let (x, y) = (teacher[i].x as usize, teacher[i].y as usize);
        if visited_j[x][y] != i64::MAX && visited_s[x][y] != i64::MAX {
            answer = answer.min(visited_j[x][y] + (visited_s[x][y] / 2));
        }
    }
    write!(out, "{}", if answer == i64::MAX { -1 } else { answer }).unwrap();
}

fn bfs(n: usize, m: usize, node: &Node, visited: &mut Vec<Vec<i64>>, board: &Vec<Vec<char>>) {
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(node.clone());
    visited[node.x as usize][node.y as usize] = 0;

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        for i in 0..4 {
            let (nx, ny) = (node.x + dx[i], node.y + dy[i]);

            if nx < 0 || nx >= n as i64 || ny < 0 || ny >= m as i64 {
                continue;
            }

            if visited[nx as usize][ny as usize] <= node.count + 2 {
                continue;
            }

            if board[nx as usize][ny as usize] == '#' {
                continue;
            }

            visited[nx as usize][ny as usize] = node.count + 2;
            queue.push_back(Node {
                x: nx,
                y: ny,
                count: node.count + 2,
            });
        }
    }
}
