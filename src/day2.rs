use std::fs;

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

pub fn solve(input_file: String, part: u8) {
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

