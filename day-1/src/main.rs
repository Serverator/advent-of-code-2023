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

    let mut a = 0;
    for line in input.lines() {
        let mut iter = line.chars().filter(|x| x.is_numeric());

        let first = iter.next().unwrap();
        let last = iter.last().unwrap_or(first);
        let mut together = first.to_string();
        together.push(last);

        a += together.parse::<i32>().unwrap();
    }
    
    println!("{}", a);
    None
}