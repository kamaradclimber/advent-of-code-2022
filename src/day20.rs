use std::fs;
use itertools::Itertools;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut array = vec![];
    let len = contents.lines().count();
    for (index, line) in lines.enumerate() {
        let n = Number {
            original_pos: index,
            shift: line.parse().unwrap(),
            left_number_id: if index > 0 { index - 1 } else { len - 1 },
            right_number_id: if index < len - 1 { index + 1 } else { 0 },
            generation: 0,
        };
        array.push(n);
    }

    let mut first_element_id = 0;
    print(&array, first_element_id);
    for i in 0..array.len() {
        let el = array[i];
        println!("===Move {0}", el.shift);
        if el.shift > 0 {
            if el == array[first_element_id] {
                first_element_id = el.right_number_id;
            }
            for _ in 0..el.shift {
                let el = array[i]; // its important we take the latest version of the element
                let current_left = el.left_number_id;
                let current_right = el.right_number_id;
                let new_right = array[current_right].right_number_id;
                println!("My old left is {0}", array[el.left_number_id].shift);
                println!("My old right is {0}", array[el.right_number_id].shift);
                let (nl, nr) = array[current_left].bind(array[current_right]);
                array[nl.original_pos] = nl;
                array[nr.original_pos] = nr;
                let (ns, nr) = el.bind(array[new_right]);
                array[ns.original_pos] = ns;
                array[nr.original_pos] = nr;
                let (nl, ns) = array[current_right].bind(array[el.original_pos]);
                array[nl.original_pos] = nl;
                array[ns.original_pos] = ns;
                let new_el = array[el.original_pos];
                println!("My new left is {0}", array[new_el.left_number_id].shift);
                println!("My new right is {0}", array[new_el.right_number_id].shift);
                print(&array, first_element_id);
            }
        } else if el.shift < 0 {
            todo!();
        } else {
            // nothing to do
        }
    }
}

fn print(array: &Vec<Number>, first_item_id: usize) {
    let mut cur = &array[array[first_item_id].right_number_id];
    let mut a = vec![];
    a.push(first_item_id);
    while cur.original_pos != first_item_id {
        a.push(cur.original_pos);
        // println!("Next index is {0}", cur.right_number_id);
        cur = &array[cur.right_number_id];
    }
    println!("{0}", a.iter().map(|&e| {
        format!("{0}({1})" , array[e].shift, array[e].generation)
    }).join(", "));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number {
    original_pos: usize,
    shift: isize,
    left_number_id: usize,
    right_number_id: usize,
    generation: u32,
}
impl Number {

    fn bind(self, other: Number) -> (Number, Number) {
        let new_self = Number { right_number_id: other.original_pos, generation: self.generation +1, ..self};
        let new_other = Number { left_number_id: self.original_pos, generation: other.generation+1, ..other};
        (new_self, new_other)
    }
}
