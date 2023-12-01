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

        let mut vec = Vec::new();
        for i in 0..line.len() {
            let line = &line[i..];

           let num = if line.chars().next().map(|x| x.is_numeric()).unwrap_or_default() {
                line.chars().next().unwrap().to_string().parse::<i32>().unwrap()
           }
            else if line.starts_with("one") { 1 }
           else if line.starts_with("two") { 2 } 
           else if line.starts_with("three") { 3 }
           else if line.starts_with("four") { 4 }
           else if line.starts_with("five") { 5 }
           else if line.starts_with("six") { 6 } 
           else if line.starts_with("seven") { 7 }
           else if line.starts_with("eight") { 8 }
           else if line.starts_with("nine") { 9 } 
           else { continue };

           vec.push(num)
        }

        let first = vec.first().unwrap();
        let last = vec.last().unwrap();
        let mut together = first.to_string() + last.to_string().as_str();

        a += together.parse::<i32>().unwrap();
    }
    
    println!("{}", a);
    None
}