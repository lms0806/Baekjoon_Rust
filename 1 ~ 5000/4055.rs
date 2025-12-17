use std::io::Write;
use std::{io, str};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
    input
  }
}

fn main() {
  let (stdin, stdout) = (io::stdin(), io::stdout());
  let mut scan = UnsafeScanner::new(stdin.lock());
  let mut out = io::BufWriter::new(stdout.lock());

  let mut testcase = 1;
  loop {
    let n = scan.token::<usize>();

    if n == 0 {
      break;
    }

    let mut pq = BinaryHeap::new();
    for _ in 0..n {
      let (s, e) = (scan.token::<i64>(), scan.token::<i64>() - 1);

      pq.push((Reverse(e), Reverse(s)));
    }

    let mut answer = 0;
    for t in 8..24 {
      let mut slot = 2;
      let mut arr = Vec::new();

      while slot > 0 && let Some((Reverse(end), Reverse(start))) = pq.pop() {
        if start <= t && t <= end {
          answer += 1;
          slot -= 1;
        } else {
          arr.push((Reverse(end), Reverse(start)));
        }
      }

      for item in arr {
        pq.push(item);
      }
    }

    writeln!(out, "On day {} Emma can attend as many as {} parties.", testcase, answer).unwrap();
    testcase += 1;
  }
}