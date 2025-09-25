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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }

    pub fn line_eof(&mut self) -> Option<String> {
        let mut input = String::new();
        let bytes_read = self.reader.read_line(&mut input).expect("Failed read");

        if bytes_read == 0 {
            None
        } else {
            Some(input)
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    write!(
        out,
        "{}",
        match scan.token::<String>().trim() {
            "걸.걸.걸" => "더 사랑할걸, 더 참을걸, 더 즐길걸.",
            "사.우.나" => "사랑과 우정을 나누자",
            "지.화.자" => "지금부터 화합하자",
            "재.건.축" => "재미있고 건강하게, 축복하며 살자",
            "오.징.어" => "오래도록 징그럽게 어울리자",
            "해.당.화" => "해가 갈수록 당당하고 화려하게",
            "우.아.미" => "우아하고 아름다운 미래를 위하여",
            "재.개.발" => "재미있고 개성있게 발전적으로 살자",
            "사.이.다" => "사랑하자 이 세상 다 바쳐",
            "주.전.자" => "주인의식을 갖고 전문성을 갖추고 자신있게 살자",
            "나.가.자" => "나라, 가정, 자신의 발전을 위하여",
            "이.기.자" => "이런 기회를 자주 만들자",
            "청.바.지" => "청춘은 바로 지금부터",
            _ => {
                ""
            }
        }
    )
    .unwrap();
}
