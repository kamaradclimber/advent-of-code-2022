use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1]
        .trim()
        .parse::<u8>()
        .expect("Unable to read <day> as an integer");
    let part = &args[2]
        .trim()
        .parse::<u8>()
        .expect("Unable to read <part> as an integer");
    let input_file = &args[3];
    println!("Solving day {day}, part {part} with input file {input_file}");

    match (day, part) {
        (1, 1) => day1(input_file.to_string(), 1),
        (1, 2) => day1(input_file.to_string(), 3),
        (2, 1) => day2(input_file.to_string(), 1),
        (2, 2) => day2(input_file.to_string(), 2),
        (3, 1) => day3(input_file.to_string(), 1),
        (3, 2) => day3(input_file.to_string(), 2),
        (_, 1) => panic!("Solution for day {day} has not been implemented yet"),
        (_, 2) => panic!("Solution for day {day} has not been implemented yet"),
        (_, _) => panic!("There are only 2 parts per day"),
    }
}

fn day3_char2priority(char: char) -> u32 {
    let c = char as u32;
    return match c {
        65..=90 => c - 65 + 27,
        97..=122 => c - 97 + 1,
        _ => panic!("Invalid character {0}", char),
    };
}

struct Rucksack {
    line: String,
}

impl Rucksack {
    fn compartment1(&self) -> &str {
        let half = self.line.len() / 2;
        &self.line[..half]
    }
    fn compartment2(&self) -> &str {
        let half = self.line.len() / 2;
        &self.line[half..]
    }
    fn unique_items(&self) -> HashSet<char> {
        let mut set = HashSet::new();
        for char in self.line.chars() {
            set.insert(char);
        }
        return set;
    }
    fn priority(&self) -> u32 {
        let mut set = HashSet::new();
        for char in self.compartment1().chars() {
            set.insert(char);
        }
        for char in self.compartment2().chars() {
            if set.contains(&char) {
                return day3_char2priority(char);
            }
        }
        panic!("No duplicate in {0}", self.line);
    }
}

fn day3(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut sum = 0;
    if part == 1 {
        for line in lines {
            let sack = Rucksack {
                line: line.to_string(),
            };
            sum += sack.priority();
        }
    } else {
        let mut elf_id = 0;
        let mut items = HashMap::<char, u8>::new();
        for line in lines {
            let sack = Rucksack {
                line: line.to_string(),
            };
            for item in sack.unique_items() {
                let count = items.entry(item).or_insert(0);
                *count += 1;
                if items[&item] == 3 {
                    sum += day3_char2priority(item);
                    break;
                }
            }

            elf_id += 1;
            if elf_id % 3 == 0 {
                items = HashMap::<char, u8>::new();
            }
        }
    }
    println!("Solution is {sum}");
}

enum Play {
    Rock,
    Paper,
    Scissors,
}

fn day2_score(player1: Play, player2: Play) -> u32 {
    match (player1, player2) {
        (Play::Rock, Play::Rock) => 1 + 3,
        (Play::Rock, Play::Paper) => 2 + 6,
        (Play::Rock, Play::Scissors) => 3 + 0,
        (Play::Paper, Play::Rock) => 1 + 0,
        (Play::Paper, Play::Paper) => 2 + 3,
        (Play::Paper, Play::Scissors) => 3 + 6,
        (Play::Scissors, Play::Rock) => 1 + 6,
        (Play::Scissors, Play::Paper) => 2 + 0,
        (Play::Scissors, Play::Scissors) => 3 + 3,
    }
}

enum Outcome {
    LOOSE,
    DRAW,
    WIN,
}

fn day2_strategy(player1: &Play, desired_outcome: Outcome) -> Play {
    match (player1, desired_outcome) {
        (Play::Rock, Outcome::DRAW) => Play::Rock,
        (Play::Rock, Outcome::LOOSE) => Play::Scissors,
        (Play::Rock, Outcome::WIN) => Play::Paper,
        (Play::Paper, Outcome::DRAW) => Play::Paper,
        (Play::Paper, Outcome::LOOSE) => Play::Rock,
        (Play::Paper, Outcome::WIN) => Play::Scissors,
        (Play::Scissors, Outcome::DRAW) => Play::Scissors,
        (Play::Scissors, Outcome::LOOSE) => Play::Paper,
        (Play::Scissors, Outcome::WIN) => Play::Rock,
    }
}

fn day2(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut score = 0;
    for line in lines {
        let player1 = match &line[0..1] {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Invalid line {line}"),
        };
        if part == 1 {
            let player2 = match &line[2..3] {
                "X" => Play::Rock,
                "Y" => Play::Paper,
                "Z" => Play::Scissors,
                _ => panic!("Invalid line {line}"),
            };
            score += day2_score(player1, player2);
        } else {
            let desired = match &line[2..3] {
                "X" => Outcome::LOOSE,
                "Y" => Outcome::DRAW,
                "Z" => Outcome::WIN,
                _ => panic!("Invalid line {line}"),
            };
            let player2 = day2_strategy(&player1, desired);
            score += day2_score(player1, player2);
        }
    }
    println!("Score is {score}")
}

fn day1(input_file: String, top: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();

    let mut packages: Vec<u32> = Vec::new();
    let mut elf_id = 0;

    for line in lines {
        if line.is_empty() {
            elf_id += 1;
        } else {
            let current_package_value = packages.get(elf_id);
            if current_package_value == None {
                packages.push(0);
            }
            packages[elf_id] += line
                .trim()
                .parse::<u32>()
                .expect("Would have expected a number but read {line}");
        }
    }
    packages.sort();
    let mut response = 0;
    for i in 0..top {
        response += packages[packages.len() - 1 - i as usize];
    }

    println!("Sum of {top} packages is {response}");
}
