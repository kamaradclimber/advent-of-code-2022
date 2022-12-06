use itertools::Itertools;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let header_size = if part == 1 { 4 } else { 14 };
    for line in lines {
        let index = first_all_different_slice(&line, header_size);
        println!("Line is {0}, first index {1}", &line, index);
    }
}

fn first_all_different_slice(input: &str, size: usize) -> usize {
    let char_vec: Vec<char> = input.chars().collect(); // <- this is the trick to manipulate
                                                       // strings as list of chars easily
    let iter = char_vec.windows(size);
    for (index, slice) in iter.enumerate() {
        if slice.iter().all_unique() {
            return index + size;
        }
    }
    panic!("Found no unique sequence of {size} in this string");
}
