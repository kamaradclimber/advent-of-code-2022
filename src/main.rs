use std::env;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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

    let input = input_file.to_string();
    match (day, part) {
        (1, 1) => day1::solve(input, 1),
        (1, 2) => day1::solve(input, 3),
        (2, _) => day2::solve(input, *part),
        (3, _) => day3::solve(input, *part),
        (4, _) => day4::solve(input, *part),
        (5, _) => day5::solve(input, *part),
        (6, _) => day6::solve(input, *part),
        (7, _) => day7::solve(input, *part),
        (8, _) => day8::solve(input, *part),
        (9, _) => day9::solve(input, *part),
        (10, _) => day10::solve(input, *part),
        (11, _) => day11::solve(input, *part),
        (12, _) => day12::solve(input, *part),
        (13, _) => day13::solve(input, *part),
        (14, _) => day14::solve(input, *part),
        (15, _) => day15::solve(input, *part),
        (16, _) => day16::solve(input, *part),
        (17, _) => day17::solve(input, *part),
        (18, _) => day18::solve(input, *part),
        (19, _) => day19::solve(input, *part),
        (20, _) => day20::solve(input, *part),
        (21, _) => day21::solve(input, *part),
        (22, _) => day22::solve(input, *part),
        (23, _) => day23::solve(input, *part),
        (24, _) => day24::solve(input, *part),
        (25, _) => day25::solve(input, *part),
        (_, _) => panic!("There are only 2 parts per day"),
    }
}
