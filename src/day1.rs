use std::fs;

pub fn solve(input_file: String, top: u8) {
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
