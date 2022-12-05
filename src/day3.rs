use std::collections::{HashMap, HashSet};
use std::fs;

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

pub fn solve(input_file: String, part: u8) {
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
