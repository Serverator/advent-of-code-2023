fn main() {
    let run_example = std::env::args().any(|arg| arg == "--ex");
    let part_two = std::env::args().any(|arg| arg == "--p2");

    let input = if run_example {
        if part_two {
            std::fs::read_to_string("./example_2.txt").unwrap()
        } else {
            std::fs::read_to_string("./example_1.txt").unwrap()
        }
    } else {
        std::fs::read_to_string("./input.txt").unwrap()
    };

    if part_two {
        part2(&input);
    } else {
        part1(&input);
    }
    
}

fn part1(input: &str) {
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
}

fn part2(input: &str) {
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
        let together = first.to_string() + last.to_string().as_str();

        a += together.parse::<i32>().unwrap();
    }
    
    println!("{}", a);
}