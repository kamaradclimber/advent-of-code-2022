use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut results = HashMap::new();
    let mut computations = VecDeque::new();
    let scalar = Regex::new(r"^(\w+): (\d+)$").unwrap();
    let operation = Regex::new(r"^(\w+): (\w+) (.) (\w+)$").unwrap();
    let mut left_root_operand = ComputationId::none();
    let mut right_root_operand = ComputationId::none();
    for line in lines {
        // println!("Line is {0}", &line);
        if scalar.is_match(line) {
            let capture = scalar.captures(line).unwrap();
            let id = ComputationId::Id(String::from(&capture[1]));
            let value: i64 = capture[2].parse().unwrap();
            results.insert(id, value);
        }
        if operation.is_match(line) {
            let capture = operation.captures(line).unwrap();
            let id = ComputationId::Id(String::from(&capture[1]));
            let op1 = ComputationId::Id(String::from(&capture[2]));
            let op2 = ComputationId::Id(String::from(&capture[4]));
            if part == 2 && id == ComputationId::Id(String::from("root")) {
                left_root_operand = op1.clone();
                right_root_operand = op2.clone();
                let computation = Computation::Equality(op1, op2);
                computations.push_back((id, computation));
            } else {
                let computation = match &capture[3] {
                    "+" => Computation::Addition(op1, op2),
                    "-" => Computation::Substraction(op1, op2),
                    "*" => Computation::Multiplication(op1, op2),
                    "/" => Computation::Division(op1, op2),
                    _ => panic!("Unsupported operation"),
                };
                computations.push_back((id, computation));
            }
        }
    }
    // parsing over let's compute
    let mut res = 0;
    let root_id = ComputationId::Id(String::from("root"));
    if part == 1 {
        res = compute_root(&root_id, &mut computations, &mut results);
    } else {
        // by observing data we see the right part of the equality test is constant
        // and the left side is nearly monotonous (increasing in the case of the demo, decreasing
        // in case of the real input)
        println!("We expect {0:?} to have a constant value", right_root_operand);
        let mut right_operand_value = None;
        let mut max_pow = 12;
        loop {
            let mut computations_for_this_round = VecDeque::new();
            for (id, computation) in &computations {
                computations_for_this_round.push_back((id.clone(), computation.clone()));
            }

            let mut results_for_this_round = HashMap::new();
            for (id, res) in &results {
                results_for_this_round.insert(id.clone(), *res);
            }
            results_for_this_round.insert(ComputationId::Id(String::from("humn")), res);
            let equal = compute_root(&root_id, &mut computations_for_this_round, &mut results_for_this_round);

            let rov = *results_for_this_round.get(&right_root_operand).unwrap();
            if right_operand_value.is_none() {
                right_operand_value = Some(rov);
            } else {
                assert_eq!(right_operand_value, Some(rov));
            }
            if equal == 1 {
                break;
            }
            let left_value = *results_for_this_round.get(&left_root_operand).unwrap();
            let sign = if left_value > rov { 1 } else { -1 };
            for pow in (3..max_pow).rev() {
                let base: i64 = 10;
                if (left_value - rov).abs() > base.pow(pow) {
                    res += base.pow(pow) * sign;
                    break;
                }
            }
            if left_value < rov {
                max_pow -= 1;
                println!("Decreasing max_pow to {max_pow}");
            }

            res += 1;
        }
    }
    println!("Solution for {part} is {res}");
}

fn compute_root(computation_id: &ComputationId, computations: &mut VecDeque<(ComputationId, Computation)>, results: &mut HashMap<ComputationId, i64>) -> i64 {
    loop {
        if computations.len() == 0 {
            break;
        }
        let (id, computation) = computations.pop_front().expect("We always have some computation to do!");
        // println!("Trying to solve {0:?} {1:?}", id, computation);
        match computation.solve(&results) {
            None => {
                // not solvable yet
                computations.push_back((id, computation));
            }
            Some(result) => {
                // println!("Found result for {0:?}: {1}", id, result);
                results.insert(id, result);
            }
        }
    }
    *results.get(computation_id).expect("We solved all computations before reaching there")
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum ComputationId {
    Id(String),
}

impl ComputationId {
    fn none() -> ComputationId {
        ComputationId::Id(String::from("this is a fake id acting as a magic value"))
    }
}

impl std::fmt::Debug for ComputationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let ComputationId::Id(s) = self;
        write!(f, "{0}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Computation {
    Addition(ComputationId, ComputationId),
    Substraction(ComputationId, ComputationId),
    Multiplication(ComputationId, ComputationId),
    Division(ComputationId, ComputationId),
    Equality(ComputationId, ComputationId),
}

impl Computation {
    fn solve(&self, results: &HashMap<ComputationId, i64>) -> Option<i64> {
        match self {
            Computation::Addition(a, b) => {
                let op1 = results.get(&a);
                let op2 = results.get(&b);
                if op1.is_some() && op2.is_some() {
                    Some(op1.unwrap() + op2.unwrap())
                } else {
                    None
                }
            }
            Computation::Substraction(a, b) => {
                let op1 = results.get(&a);
                let op2 = results.get(&b);
                if op1.is_some() && op2.is_some() {
                    Some(op1.unwrap() - op2.unwrap())
                } else {
                    None
                }
            }
            Computation::Multiplication(a, b) => {
                let op1 = results.get(&a);
                let op2 = results.get(&b);
                if op1.is_some() && op2.is_some() {
                    Some(op1.unwrap() * op2.unwrap())
                } else {
                    None
                }
            }
            Computation::Division(a, b) => {
                let op1 = results.get(&a);
                let op2 = results.get(&b);
                if op1.is_some() && op2.is_some() {
                    Some(op1.unwrap() / op2.unwrap())
                } else {
                    None
                }
            }
            Computation::Equality(a, b) => {
                let op1 = results.get(&a);
                let op2 = results.get(&b);
                if op1.is_some() && op2.is_some() {
                    // println!("{0} == {1}", op1.unwrap(), op2.unwrap());
                    Some(if op1.unwrap() == op2.unwrap() { 1 } else { 0 })
                } else {
                    None
                }
            }
        }
    }
}
