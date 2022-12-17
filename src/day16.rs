use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut result = 0;
    for line in lines {
        result += 1;
        println!("Line is {0}", &line);
    }
    println!("Score for part {0}, is {result}", part);
}
