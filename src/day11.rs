use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let blocks = contents.split("\n\n");
    for block in blocks {
        let monkey_lines : Vec<&str> = block.lines().collect();
        println!("Monkey is {0}", &monkey_lines[0]);
    }

    let mut monkey_holdings : Vec<Vec<Item>> = vec![];
    let mut monkeys : Vec<Monkey> = vec![];

    let m0 = Monkey {
        operation: ItemOperation {
            pre_examination: |old| old * 19,
            post_examination: |old| old / 3
        },
        test: ItemTestDestination {
            test: |i| i % 23 == 0,
            destination_if_true: 2,
            destination_if_false: 3
        }
    };
    dbg!(&m0);
    let pre = m0.operation.pre_examination;
    let post = m0.operation.post_examination;

    dbg!(post(dbg!(pre(Item(79)))));
}

#[derive(Debug)]
struct Item(u32);

impl std::ops::Mul<u32> for Item {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        let Item(worry_level) = self;
        Item(worry_level * rhs)
    }
}

impl std::ops::Div<u32> for Item {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        let Item(worry_level) = self;
        Item(worry_level / rhs)
    }
}
impl std::ops::Rem<u32> for Item {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        let Item(worry_level) = self;
        worry_level % rhs
    }
}

#[derive(Debug)]
struct ItemOperation {
    pre_examination: fn(Item) -> Item,
    post_examination: fn(Item) -> Item,
}

#[derive(Debug)]
struct ItemTestDestination {
    test: fn(Item) -> bool,
    destination_if_true: usize,
    destination_if_false: usize,
}

#[derive(Debug)]
struct Monkey {
    operation: ItemOperation,
    test: ItemTestDestination,
}
