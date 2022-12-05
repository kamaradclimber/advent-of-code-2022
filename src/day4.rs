use regex::Regex;
use std::fs;

fn includes_range(
    me: &std::ops::RangeInclusive<&u32>,
    other: &std::ops::RangeInclusive<&u32>,
) -> bool {
    me.start() <= other.start() && me.end() >= other.end()
}
fn overlap_range(
    me: &std::ops::RangeInclusive<&u32>,
    other: &std::ops::RangeInclusive<&u32>,
) -> bool {
    other.contains(&me.start())
        || other.contains(&me.end())
        || me.contains(&other.start())
        || me.contains(&other.end())
}

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut sum = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    for line in lines {
        let cap = re.captures(line).unwrap();
        let start1 = &cap[1].parse::<u32>().unwrap();
        let end1 = &cap[2].parse::<u32>().unwrap();
        let start2 = &cap[3].parse::<u32>().unwrap();
        let end2 = &cap[4].parse::<u32>().unwrap();
        let elf1 = std::ops::RangeInclusive::new(start1, end1);
        let elf2 = std::ops::RangeInclusive::new(start2, end2);
        if part == 1 && (includes_range(&elf1, &elf2) || includes_range(&elf2, &elf1)) {
            sum += 1;
        }
        if part == 2 && overlap_range(&elf1, &elf2) {
            sum += 1;
        }
    }
    println!("Answer is {sum}");
}
