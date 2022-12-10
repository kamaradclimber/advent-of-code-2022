use itertools::Itertools;
use std::collections::HashSet;
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
        self.line.chars().collect()
    }
    fn priority(&self) -> u32 {
        let set: HashSet<char> = self.compartment1().chars().collect();
        let my_char = self
            .compartment2()
            .chars()
            .find(|c| set.contains(c))
            .expect("No duplicate found");
        day3_char2priority(my_char)
    }
}

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    if part == 1 {
        let sum: u32 = lines
            .map(|line| {
                Rucksack {
                    line: line.to_string(),
                }
                .priority()
            })
            .sum();
        println!("Solution is {sum}");
    } else {
        let sum: u32 = lines
            .map(|line| Rucksack {
                line: line.to_string(),
            })
            .collect::<Vec<Rucksack>>()
            .chunks(3)
            .map(|sacks| {
                let item_frequencies = sacks.iter().flat_map(Rucksack::unique_items).counts();
                let (common_item, _) = item_frequencies
                    .iter()
                    .find(|(_char, count)| **count == 3)
                    .expect("Elf group does not have a common item");
                day3_char2priority(*common_item)
            })
            .sum();
        println!("Solution is {sum}");
    }
}
