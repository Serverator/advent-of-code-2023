use std::collections::HashMap;

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
    let mut sum = 0;
    'game: for game in input.lines() {
        let (id,game) = game.split_once(':').unwrap();

        for cube in game.split(&[';',',']) {
            let (amount, color) = cube.trim().split_once(' ').unwrap();
            let amount = amount.trim().parse::<u32>().unwrap();
            let possible = match color {
                "red" => amount <= 12,
                "green" => amount <= 13,
                "blue" => amount <= 14,
                _ => continue,
            };

            if !possible {
                continue 'game;
            }
        }

        let id = id.trim().split_once(' ').unwrap().1.parse::<u32>().unwrap();

        sum += id;
    }
    println!("{}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for game in input.lines() {
        let game = game.split_once(':').unwrap().1.trim();
        let mut cubes = HashMap::new();
        for cube in game.split(&[';',',']) {
            let (amount, color) = cube.trim().split_once(' ').unwrap();
            let amount = amount.trim().parse::<u32>().unwrap();
            let entry = cubes.entry(color.to_string()).or_insert(amount);

            if *entry < amount {
                *entry = amount
            };
        }

        let cube_sum = cubes.values().fold(1, |mut acc, x| {acc *= *x; acc});
        sum += cube_sum;
    }
    println!("{}", sum);
}