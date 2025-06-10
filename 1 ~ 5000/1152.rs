use std::io::{read_to_string, stdin};

fn main() {
    println!(
        "{}",
        read_to_string(stdin()).unwrap().split_whitespace().count()
    );
}
