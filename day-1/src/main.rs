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
    let result = input.lines().map(|l| {
        let vec: Vec<_> = l.chars().filter(|c| c.is_numeric()).collect();
        format!("{}{}", vec.first().unwrap(), vec.last().unwrap()).parse::<i32>().unwrap()
    }).sum::<i32>();

    println!("{}", result);
}

fn part2(input: &str) {
    let result = input.lines().map(|l| {
        let mut vec = Vec::new();
        for i in 0..l.len() {
            let l = &l[i..];

            let num = if l.chars().next().map(|x| x.is_numeric()).unwrap_or_default() {
                 l.chars().next().unwrap().to_digit(10).unwrap()
            }
            else if l.starts_with("one") { 1 }
            else if l.starts_with("two") { 2 } 
            else if l.starts_with("three") { 3 }
            else if l.starts_with("four") { 4 }
            else if l.starts_with("five") { 5 }
            else if l.starts_with("six") { 6 } 
            else if l.starts_with("seven") { 7 }
            else if l.starts_with("eight") { 8 }
            else if l.starts_with("nine") { 9 } 
            else { continue };
        
            vec.push(num)
        }
        format!("{}{}", vec.first().unwrap(), vec.last().unwrap()).parse::<i32>().unwrap()
    }).sum::<i32>();

    println!("{}", result);
}