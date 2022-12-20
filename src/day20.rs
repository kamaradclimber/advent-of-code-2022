use itertools::Itertools;
use std::fs;

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
        };
        array.push(n);
    }

    let mut first_element_id = 0;
    // print(&array, first_element_id);
    for i in 0..array.len() {
        let el = array[i];
        // println!("===Move {0} ({1}/{2})", el.shift, i+1, array.len());
        if el.shift > 0 {
            for _ in 0..el.shift {
                let el = array[i]; // its important we take the latest version of the element
                if el == array[first_element_id] {
                    first_element_id = el.right_number_id;
                }
                let current_left = el.left_number_id;
                let current_right = el.right_number_id;
                let new_right = array[current_right].right_number_id;
                let (nl, nr) = array[current_left].bind(array[current_right]);
                array[nl.original_pos] = nl;
                array[nr.original_pos] = nr;
                let (ns, nr) = el.bind(array[new_right]);
                array[ns.original_pos] = ns;
                array[nr.original_pos] = nr;
                let (nl, ns) = array[current_right].bind(array[el.original_pos]);
                array[nl.original_pos] = nl;
                array[ns.original_pos] = ns;
                // print(&array, first_element_id);
            }
        } else if el.shift < 0 {
            for _ in el.shift..0 {
                let el = array[i]; // its important we take the latest version of the element
                if el == array[first_element_id] {
                    first_element_id = el.right_number_id;
                }
                let current_left = el.left_number_id;
                let current_right = el.right_number_id;
                let new_left = array[current_left].left_number_id;
                let (a, b) = array[new_left].bind(array[el.original_pos]);
                array[a.original_pos] = a;
                array[b.original_pos] = b;
                let (a, b) = array[el.original_pos].bind(array[current_left]);
                array[a.original_pos] = a;
                array[b.original_pos] = b;
                let (a, b) = array[current_left].bind(array[current_right]);
                array[a.original_pos] = a;
                array[b.original_pos] = b;
                // print(&array, first_element_id);
            }
        } else {
            // nothing to do
        }
        // print(&array, first_element_id);
    }

    let index_of_0 = array.iter().enumerate().find(|&(_, number)| number.shift == 0).unwrap().0;
    let mut sum = 0;
    let mut current_index = index_of_0;
    for i in 1..=3000 {
        current_index = array[current_index].right_number_id;
        if i % 1000 == 0 {
            dbg!(array[current_index].shift);
            sum += array[current_index].shift;
        }
    }
    println!("Response for part {part} is {sum}");
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
    println!("{0}", a.iter().map(|&e| { format!("{0}", array[e].shift) }).join(", "));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number {
    original_pos: usize,
    shift: isize,
    left_number_id: usize,
    right_number_id: usize,
}
impl Number {
    fn bind(self, other: Number) -> (Number, Number) {
        let new_self = Number {
            right_number_id: other.original_pos,
            ..self
        };
        let new_other = Number {
            left_number_id: self.original_pos,
            ..other
        };
        (new_self, new_other)
    }
}
