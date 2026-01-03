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

  let n = scan.token::<usize>();

  let mut arr = vec![0; n + 1];
  for i in 1..=n {
    arr[i] = scan.token::<i64>() * 100;
  }

  let len = scan.token::<i64>() * 100;

  let mut sum = vec![0; n + 1];
  for i in 1..=n {
    sum[i] = sum[i - 1] + arr[i];
  }

  let max = if sum[n] > len {
    sum[n] - len
  } else {
    0
  };

  let num = len / 2;

  for _ in 0..scan.token::<usize>() {
    let idx = scan.token::<usize>();
    let answer = ((sum[idx - 1] + arr[idx] / 2) - num).max(0).min(max);

    writeln!(out, "{}.{:02}", answer / 100, answer % 100).unwrap();
  }
}