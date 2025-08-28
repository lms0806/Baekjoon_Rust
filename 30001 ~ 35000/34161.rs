use std::io;
use std::io::Write;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());

    for _ in 0..10000 {
        writeln!(out, "-1").unwrap();
    }
}
