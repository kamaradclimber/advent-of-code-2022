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
    for line in lines {
        println!("Line is {0}", &line);
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
    // parsing over let's compute
    loop {
        if computations.len() == 0 {
            break;
        }
        let (id, computation) = computations.pop_front().expect("We always have some computation to do!");
        println!("Trying to solve {0:?} {1:?}", id, computation);
        match computation.solve(&results) {
            None => {
                // not solvable yet
                computations.push_back((id, computation));
            }
            Some(result) => {
                println!("Found result for {0:?}: {1}", id, result);
                results.insert(id, result);
            }
        }
    }
    let res = results.get(&ComputationId::Id(String::from("root"))).expect("We solved all computations before reaching there");
    println!("Result for {part} is {res}");
}

#[derive(PartialEq, Eq, Hash)]
enum ComputationId {
    Id(String),
}

impl std::fmt::Debug for ComputationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let ComputationId::Id(s) = self;
        write!(f, "{0}", s)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Computation {
    Addition(ComputationId, ComputationId),
    Substraction(ComputationId, ComputationId),
    Multiplication(ComputationId, ComputationId),
    Division(ComputationId, ComputationId),
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
                    println!("{0} * {1}", op1.unwrap(), op2.unwrap());
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
        }
    }
}
