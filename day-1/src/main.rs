use std::time::Duration;

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

const DIGITS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part2(input: &str) {
    let result = input.lines().map(|l| {
        let mut vec = Vec::new();
        for i in 0..l.len() {
            let l = &l[i..];

            if let Some(char) = l.chars().next() {
                if char.is_numeric() {
                    vec.push(char.to_digit(10).unwrap());
                    continue;
                }
            }

            for (digit, num) in DIGITS {
                if l.starts_with(digit) {
                    vec.push(*num);
                }
            }
        }
        format!("{}{}", vec.first().unwrap(), vec.last().unwrap()).parse::<i32>().unwrap()
    }).sum::<i32>();

    println!("{}", result);
}