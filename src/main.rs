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
        (2, 1) => day2(input_file.to_string()),
        (_, 1) => panic!("Solution for day {day} has not been implemented yet"),
        (_, 2) => panic!("Solution for day {day} has not been implemented yet"),
        (_, _) => panic!("There are only 2 parts per day"),
    }
}

fn day2(input_file: String) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut score = 0;
    for line in lines {
        match line {
            "A X" => score += 1 + 3,
            "A Y" => score += 2 + 6,
            "A Z" => score += 3 + 0,
            "B X" => score += 1 + 0,
            "B Y" => score += 2 + 3,
            "B Z" => score += 3 + 6,
            "C X" => score += 1 + 6,
            "C Y" => score += 2 + 0,
            "C Z" => score += 3 + 3,
            _ => panic!("Unknown line {line}"),
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
