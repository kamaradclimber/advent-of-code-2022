use itertools::Itertools;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    for line in lines {
        let index = first_all_different_slice(&line, 4);
        println!("Line is {0}, first index {1}", &line, index);
    }
}

fn first_all_different_slice(input: &str, size: usize) -> usize {
    let char_vec: Vec<char> = input.chars().collect(); // <- this is the trick to manipulate
                                                       // strings as list of chars easily
    let iter = char_vec.windows(size);
    for (index, slice) in iter.enumerate() {
        if slice.iter().all_unique() {
            return index + 4;
        }
    }
    panic!("Found no unique sequence of {size} in this string");
}
