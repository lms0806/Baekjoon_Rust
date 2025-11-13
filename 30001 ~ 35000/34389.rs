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

    let plan = [
        ("Marble", 19, 200.0),
        ("Marble+", 19, 350.0),
        ("Quartz", 14, 200.0),
        ("Quartz+", 14, 350.0),
    ];

    for _ in 0..scan.token::<usize>() {
        let (name, plan_name, swipe, money) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<usize>(),
            scan.token::<f64>(),
        );

        let (_, plan_swipe, plan_money) = plan.iter().find(|(p, _, _)| *p == plan_name).unwrap();

        let (need_swipe, need_money) = ((plan_swipe - swipe).max(0), (plan_money - money).max(0.0));

        writeln!(
            out,
            "{} {} {:.2} {}",
            name,
            need_swipe,
            need_money,
            match (need_swipe > 0, need_money > 0.0) {
                (true, true) => "Use meal swipe or munch money",
                (true, false) => "Use meal swipe",
                (false, true) => "Use munch money",
                (false, false) => "Go to Downtown Golden!",
            }
        )
        .unwrap();
    }
}
