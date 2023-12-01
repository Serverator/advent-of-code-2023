use std::fmt::Debug;

fn main() {
    let run_example = std::env::args().any(|arg| arg == "--ex");

    let input = if run_example {
        std::fs::read_to_string("./example.txt").unwrap()
    } else {
        std::fs::read_to_string("./input.txt").unwrap()
    };

    compute(&input);
}

fn compute(input: &str) -> Option<()> {

    println!("{}", input);

    None
}