use regex::Regex;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let mut monkey_holdings: Vec<Vec<Item>> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];
    let blocks = contents.split("\n\n");
    for (id, block) in blocks.enumerate() {
        let monkey_lines: Vec<&str> = block.lines().collect();
        monkey_holdings.push(parse_item_line(&monkey_lines[1]));
        let operation = monkey_lines[2].trim().parse::<ItemOperation>().unwrap();
        let test = monkey_lines[3..6]
            .join("\n")
            .parse::<ItemTestDestination>()
            .unwrap();
        let monkey = Monkey {
            id,
            operation,
            test,
        };
        monkeys.push(monkey);
    }
    let mut inspected_items: Vec<u64> = vec![0; monkeys.len()];

    let round = if part == 1 { 20 } else { 10000 };

    // technically we could take the gcd of all those numbers but they are prime numbers 🤯
    let ring: u64 = monkeys.iter().map(|m| m.test.divide_criteria).product();
    println!("🤓 Using {ring} as a way to keep worry level manageable");

    for round in 1..=round {
        for monkey in monkeys.iter() {
            let holdings = &monkey_holdings[monkey.id];
            let mut new_holdings: Vec<Vec<Item>> = vec![vec![]; monkey_holdings.len()];
            for item in holdings {
                inspected_items[monkey.id] += 1;
                let new_item = if part == 1 {
                    monkey.operation.run(*item) / 3
                } else {
                    Item(monkey.operation.run(*item) % ring)
                };
                let dest_id = if (new_item) % monkey.test.divide_criteria == 0 {
                    monkey.test.destination_if_true
                } else {
                    monkey.test.destination_if_false
                };
                new_holdings[dest_id].push(new_item);
                // println!("Monkey {0}: Item with worry level {1:?} is thrown to monkey {dest_id} (was {2:?} before)", monkey.id, new_item, initial_worry_level);
            }
            monkey_holdings[monkey.id].clear();
            // TODO: find a way to avoid this intermediate structure to pass transferred items
            for id in vec![
                monkey.test.destination_if_true,
                monkey.test.destination_if_false,
            ] {
                monkey_holdings[id].append(&mut new_holdings[id]);
            }
        }
        if round == 1 {
            inspect_monkey_actions(round, &inspected_items, &monkey_holdings);
        }
        if round == 20 {
            inspect_monkey_actions(round, &inspected_items, &monkey_holdings);
        }
        if round % 1000 == 0 {
            inspect_monkey_actions(round, &inspected_items, &monkey_holdings);
        }
    }
    inspected_items.sort();
    let inspected_items: Vec<&u64> = inspected_items.iter().rev().collect();
    let monkey_business_level = inspected_items[0] * inspected_items[1];
    println!("Level of monkey business is {monkey_business_level}");
}

fn inspect_monkey_actions(
    round: u64,
    inspected_items: &Vec<u64>,
    monkey_holdings: &Vec<Vec<Item>>,
) {
    println!("== After round {round} ==");
    for (id, item_count) in inspected_items.iter().enumerate() {
        println!("Monkey {0} inspect items {1} times.", id, item_count);
        // dbg!(&monkey_holdings[id]);
    }
}

fn parse_item_line(line: &str) -> Vec<Item> {
    let re = Regex::new(r"^Starting items: (.+)$").unwrap();
    let cap = re.captures(line.trim()).unwrap();
    let numbers: Vec<&str> = cap[1].split(", ").collect();
    let mut my_numbers = vec![];
    for n in numbers {
        let i: Item = Item(n.parse::<u64>().unwrap());
        my_numbers.push(i);
    }
    my_numbers
}

#[derive(Debug, Clone, Copy)]
struct Item(u64);

impl std::ops::Add<u64> for Item {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        let Item(worry_level) = self;
        Item(worry_level + rhs)
    }
}

impl std::ops::Mul<u64> for Item {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        let Item(worry_level) = self;
        Item(worry_level * rhs)
    }
}
impl std::ops::Mul<Item> for Item {
    type Output = Self;

    fn mul(self, rhs: Item) -> Self::Output {
        let Item(worry_level) = rhs;
        self * worry_level
    }
}

impl std::ops::Div<u64> for Item {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        let Item(worry_level) = self;
        Item(worry_level / rhs)
    }
}
impl std::ops::Rem<u64> for Item {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        let Item(worry_level) = self;
        worry_level % rhs
    }
}

#[derive(Debug)]
enum ItemOperation {
    Square,
    Add(u64),
    Mul(u64),
}

impl ItemOperation {
    fn run(&self, old: Item) -> Item {
        match self {
            ItemOperation::Square => old * old,
            ItemOperation::Add(el) => old + *el,
            ItemOperation::Mul(el) => old * *el,
        }
    }
}

#[derive(Debug)]
enum InstructionParsingError {
    UnknownInstruction,
}

impl std::str::FromStr for ItemOperation {
    type Err = InstructionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Operation: new = old * old" {
            return Ok(ItemOperation::Square);
        }
        let re = Regex::new(r"^Operation: new = old (.) (\d+)$").unwrap();
        if !re.is_match(s) {
            return Err(InstructionParsingError::UnknownInstruction);
        }
        let cap = re.captures(s).unwrap();
        let operand: u64 = cap[2].parse().unwrap();
        let operator = &cap[1];
        match operator {
            "+" => Ok(ItemOperation::Add(operand)),
            "*" => Ok(ItemOperation::Mul(operand)),
            _ => Err(InstructionParsingError::UnknownInstruction),
        }
    }
}

#[derive(Debug)]
struct ItemTestDestination {
    divide_criteria: u64,
    destination_if_true: usize,
    destination_if_false: usize,
}

impl std::str::FromStr for ItemTestDestination {
    type Err = InstructionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Test: divisible by (\d+)\n.+If true: throw to monkey (\d+)\n.+If false: throw to monkey (\d+)$").unwrap();
        if !re.is_match(s.trim()) {
            return Err(InstructionParsingError::UnknownInstruction);
        }
        let numbers = re.captures(s.trim()).unwrap();
        let divide_criteria: u64 = numbers[1].parse().unwrap();
        let destination_if_true: usize = numbers[2].parse().unwrap();
        let destination_if_false: usize = numbers[3].parse().unwrap();
        Ok(ItemTestDestination {
            divide_criteria,
            destination_if_true,
            destination_if_false,
        })
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    operation: ItemOperation,
    test: ItemTestDestination,
}
