use itertools::Itertools;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let all_moves = lines.flat_map(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        let my_move: Move = Move::from(words[0]);
        let count: usize = words[1].parse().expect("Would expect a number of move");
        vec![my_move; count]
    });
    let rope_size = if part == 1 { 2 } else { 10 };
    let mut rope: Vec<Position> = vec![];
    for i in 0..rope_size {
        rope.push(Position { x: 0, y: 0 });
    }

    let mut successive_tail_positions = vec![];

    for my_move in all_moves {
        rope[0] = rope[0] + my_move;
        // let's compute new tail position
        for i in 1..rope_size {
            match distance(rope[i], rope[i - 1]) {
                0 | 1 => (),
                _ => {
                    // tail has to move
                    let dx = rope[i - 1].x - rope[i].x;
                    match dx {
                        1 | 2 => rope[i] = rope[i] + Move::R,
                        -1 | -2 => rope[i] = rope[i] + Move::L,
                        0 => (),
                        _ => panic!("Tail and Head should not be at more than 2 x apart"),
                    };
                    let dy = rope[i - 1].y - rope[i].y;
                    match dy {
                        1 | 2 => rope[i] = rope[i] + Move::D,
                        -1 | -2 => rope[i] = rope[i] + Move::U,
                        0 => (),
                        _ => panic!("Tail and Head should not be at more than 2 y apart"),
                    };
                }
            }
        }
        successive_tail_positions.push(rope[rope_size - 1]);
    }

    let unique_positions = successive_tail_positions.iter().unique().count();
    println!("Solution for part {part} is {unique_positions} unique position for tail");
}

fn distance(p1: Position, p2: Position) -> u32 {
    let dx = (p1.x - p2.x).abs() as u32;
    let dy = (p1.y - p2.y).abs() as u32;
    std::cmp::max(dx, dy)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
enum Move {
    R,
    L,
    U,
    D,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "R" => Move::R,
            "L" => Move::L,
            "U" => Move::U,
            "D" => Move::D,
            _ => panic!("Invalid input movement"),
        }
    }
}

impl std::ops::Add<Move> for Position {
    type Output = Position;

    fn add(self, movement: Move) -> Position {
        match movement {
            Move::R => Position {
                x: self.x + 1,
                ..self
            },
            Move::L => Position {
                x: self.x - 1,
                ..self
            },
            Move::U => Position {
                y: self.y - 1,
                ..self
            },
            Move::D => Position {
                y: self.y + 1,
                ..self
            },
        }
    }
}
