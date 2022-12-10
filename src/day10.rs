use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut register_x_values = vec![1];
    let snapshot_points: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    for line in lines {
        let instruction: Instruction = line.parse().unwrap();
        match instruction {
            Instruction::Noop => register_x_values.push(
                *register_x_values
                    .last()
                    .expect("Register has always a value"),
            ),
            Instruction::Addx(operand) => {
                let last_value = *register_x_values
                    .last()
                    .expect("Register has always a value");
                register_x_values.push(last_value);
                register_x_values.push(last_value + operand);
            }
        }
    }
    if part == 1 {
        let res: i32 = snapshot_points
            .iter()
            .map(|&cycle| signal_strength(&register_x_values, cycle))
            .sum();
        println!("Sum of signal strength is {res}");
    } else {
        for cycle in 1..=240 {
            let pixel_position = (cycle as i32 - 1) % 40;
            let c = if pixel_position >= register_x_values[cycle - 1] - 1
                && pixel_position <= register_x_values[cycle - 1] + 1
            {
                "#"
            } else {
                "."
            };
            print!("{c}");
            if cycle % 40 == 0 {
                print!("\n");
            }
        }
    }
}

fn signal_strength(register_x_values: &Vec<i32>, cycle: usize) -> i32 {
    // we are looking for value *during* cycle so we need to look at end of cycle-1
    register_x_values[cycle - 1] * (cycle as i32)
}

#[derive(Debug)]
enum InstructionParsingError {
    InvalidAddx,
    UnknownInstruction,
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl std::str::FromStr for Instruction {
    type Err = InstructionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::Noop)
        } else if &s[..4] == "addx" {
            let words: Vec<&str> = s.split_whitespace().collect();
            if words.len() != 2 {
                return Err(InstructionParsingError::InvalidAddx);
            }
            match words[1].parse() {
                Err(_) => Err(InstructionParsingError::InvalidAddx),
                Ok(int) => Ok(Instruction::Addx(int)),
            }
        } else {
            dbg!(s); // TODO: we should include s in the InstructionParsingError type
                     // but I face lifetime issues 😢
            Err(InstructionParsingError::UnknownInstruction)
        }
    }
}
