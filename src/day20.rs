use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut array = vec![];
    let len = contents.lines().count();
    let decryption_key = if part == 1 { 1 } else { 811589153 };
    let mix_count = if part == 1 { 1 } else { 10 };
    for (index, line) in lines.enumerate() {
        let n = Number {
            original_pos: index,
            shift: line.parse::<isize>().unwrap() * decryption_key,
            left_number_id: if index > 0 { index - 1 } else { len - 1 },
            right_number_id: if index < len - 1 { index + 1 } else { 0 },
        };
        array.push(n);
    }

    for _ in 0..mix_count {
        for i in 0..array.len() {
            let el = array[i];
            // println!("===Move {0} ({1}/{2})", el.shift, i+1, array.len());
            let count = el.shift.abs() as usize % (array.len() - 1);
            if el.shift > 0 {
                for _ in 0..count {
                    shift_left_once(&mut array, i);
                }
            } else if el.shift < 0 {
                for _ in 0..count {
                    shift_right_once(&mut array, i);
                }
            } else {
                // nothing to do
            }
        }
    }

    let index_of_0 = array.iter().enumerate().find(|&(_, number)| number.shift == 0).unwrap().0;
    let mut sum = 0;
    let mut current_index = index_of_0;
    for i in 1..=3000 {
        current_index = array[current_index].right_number_id;
        if i % 1000 == 0 {
            sum += array[current_index].shift;
        }
    }
    println!("Response for part {part} is {sum}");
}
fn shift_right_once(array: &mut Vec<Number>, index: usize) {
    let el = array[index]; // its important we take the latest version of the element
    let current_left = el.left_number_id;
    let current_right = el.right_number_id;
    let new_left = array[current_left].left_number_id;
    array[new_left].bind(array[el.original_pos], array);
    array[el.original_pos].bind(array[current_left], array);
    array[current_left].bind(array[current_right], array);
}

fn shift_left_once(array: &mut Vec<Number>, index: usize) {
    let el = array[index]; // its important we take the latest version of the element
    let current_left = el.left_number_id;
    let current_right = el.right_number_id;
    let new_right = array[current_right].right_number_id;
    array[current_left].bind(array[current_right], array);
    array[el.original_pos].bind(array[new_right], array);
    array[current_right].bind(array[el.original_pos], array);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number {
    original_pos: usize,
    shift: isize,
    left_number_id: usize,
    right_number_id: usize,
}
impl Number {

    fn bind(self, other: Number, array: &mut Vec<Number>) {
        let new_self = Number {
            right_number_id: other.original_pos,
            ..self
        };
        let new_other = Number {
            left_number_id: self.original_pos,
            ..other
        };
        array[new_self.original_pos] = new_self;
        array[new_other.original_pos] = new_other;
    }
}
