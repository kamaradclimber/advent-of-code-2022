use std::env;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

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
        (1, 1) => day1::solve(input_file.to_string(), 1),
        (1, 2) => day1::solve(input_file.to_string(), 3),
        (2, 1) => day2::solve(input_file.to_string(), 1),
        (2, 2) => day2::solve(input_file.to_string(), 2),
        (3, 1) => day3::solve(input_file.to_string(), 1),
        (3, 2) => day3::solve(input_file.to_string(), 2),
        (4, 1) => day4::solve(input_file.to_string(), 1),
        (4, 2) => day4::solve(input_file.to_string(), 2),
        (5, 1) => day5::solve(input_file.to_string(), 1),
        (5, 2) => day5::solve(input_file.to_string(), 2),
        (_, 1) => panic!("Solution for day {day} has not been implemented yet"),
        (_, 2) => panic!("Solution for day {day} has not been implemented yet"),
        (_, _) => panic!("There are only 2 parts per day"),
    }
}
