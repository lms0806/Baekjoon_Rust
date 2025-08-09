use io::Write;
use std::collections::HashMap;
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

    let n = scan.token::<usize>();

    let mut map = (0..n)
        .map(|_| scan.token::<i64>())
        .fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });

    let mut map2 = (0..n)
        .map(|_| scan.token::<i64>())
        .fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });

    let arr = map
        .keys()
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|key| {
            let min_cnt = map2.get(&key).map(|&v2| v2.min(map[&key]))?;
            if min_cnt == 0 {
                return None;
            }

            // 감소 처리
            if let Some(x) = map.get_mut(&key) {
                *x -= min_cnt;
                if *x == 0 {
                    map.remove(&key);
                }
            }

            if let Some(x) = map2.get_mut(&key) {
                *x -= min_cnt;
                if *x == 0 {
                    map2.remove(&key);
                }
            }

            Some(std::iter::repeat(key).take(min_cnt))
        })
        .flatten()
        .collect::<Vec<i64>>();

    writeln!(out, "{}", arr.len()).unwrap();
    arr.iter().for_each(|x| write!(out, "{} ", x).unwrap());

    map.iter()
        .flat_map(|(&key, &count)| std::iter::repeat(key).take(count))
        .for_each(|x| write!(out, "{} ", x).unwrap());

    writeln!(out).unwrap();

    arr.iter().for_each(|x| write!(out, "{} ", x).unwrap());

    map2.iter()
        .flat_map(|(&key, &count)| std::iter::repeat(key).take(count))
        .for_each(|x| write!(out, "{} ", x).unwrap());
}
