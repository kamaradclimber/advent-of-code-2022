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

    day1_part1(input_file.to_string());
}

fn day1_part1(input_file: String) {
    println!("Starting to solve day1 part 1");

    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();

    let mut richest_package = 0;
    let mut current_package = 0;

    for line in lines {
        if line.is_empty() {
            if current_package > richest_package {
                richest_package = current_package;
                println!("Current package is {current_package}, richest package {richest_package}");
            }
            current_package = 0;
        } else {
            current_package += line
                .trim()
                .parse::<u32>()
                .expect("Would have expected a number but read {line}");
        }
    }
    if current_package > richest_package {
        richest_package = current_package;
        println!("Current package is {current_package}, richest package {richest_package}");
        current_package = 0;
    }

    println!("Richest package is {richest_package}");
}
