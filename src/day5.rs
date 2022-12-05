use regex::Regex;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let mut lines = contents.lines();
    let nb_columns = 30;
    let mut stacks = vec![Vec::new(); nb_columns + 1];

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let chars = line.as_bytes();
        if let Some(el) = chars.get(1) {
            let char = *el as char;
            if char == '1' {
                continue;
            }
        }
        for column_id in 1..=nb_columns {
            let idx = 1 + (column_id - 1) * 4;
            let my_crate = chars.get(idx);
            if let Some(my_crate) = my_crate {
                let real_char = *my_crate as char;
                if real_char != ' ' {
                    stacks[column_id].push(real_char);
                }
            }
        }
    }
    for column_id in 1..=nb_columns {
        stacks[column_id].reverse();
    }
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in lines {
        let instruction = re.captures(line).unwrap();
        let count = &instruction[1].parse::<u32>().unwrap();
        let src_column_id = &instruction[2].parse::<usize>().unwrap();
        let dst_column_id = &instruction[3].parse::<usize>().unwrap();
        if part == 1 {
            for _ in 0..*count {
                match stacks[*src_column_id].pop() {
                    Some(my_crate) => stacks[*dst_column_id].push(my_crate),
                    None => panic!("Stack is empty, cannot move element from it!"),
                }
            }
        } else {
            let mut transient = Vec::new();
            for _ in 0..*count {
                match stacks[*src_column_id].pop() {
                    Some(my_crate) => transient.push(my_crate),
                    None => panic!("Stack is empty, cannot move element from it!"),
                }
            }
            for _ in 0..*count {
                match transient.pop() {
                    Some(my_crate) => stacks[*dst_column_id].push(my_crate),
                    _ => panic!("Transient stack is empty, this is really impossible"),
                }
            }
        }
    }
    let mut stack_top = Vec::new();
    for mut stack in stacks {
        match stack.pop() {
            Some(my_crate) => stack_top.push(my_crate),
            None => (),
        }
    }
    let output: String = stack_top.iter().collect();
    println!("Solution is {output}");
}
