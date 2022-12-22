use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");

    let mut coords = HashMap::<Point, Tile>::new();
    let mut columns = Vec::<(i32,i32)>::new();
    let mut lines = Vec::<(i32,i32)>::new();
    let input_parts : Vec<&str> = contents.split("\n\n").collect();
    let input_lines = input_parts[0].lines();

    // lets use 0-index entries during the program, we'll move to 1-index only at the end

    for (y, line) in input_lines.enumerate() {
        let mut minx = 100000 as i32;
        let mut maxx = 0 as i32;
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Open),
                _ => None,
            };
            if let Some(t) = tile {
                coords.insert((x as i32,y as i32), t);
                minx = std::cmp::min(minx, x as i32);
                maxx = std::cmp::max(maxx, x as i32);
            }
        }
        lines.push((minx,maxx));
    }
    let column_count = lines.iter().map(|(_,max)| max).max().unwrap() + 1;
    for x in 0..column_count {
        let column_entries : Vec<i32> = coords.iter().filter(|(&(xx,_), _)| xx == x as i32).map(|(&(_,yy),_)| yy).collect();
        columns.push((*column_entries.iter().min().unwrap(), *column_entries.iter().max().unwrap()));
    }

    let pattern = Regex::new(r"\d+|R|L").unwrap();
    let instructions = input_parts[1];

    let mut cur = (lines[0].0 as i32, 0);
    let mut cur_direction = Direction::Right;

    for instruction in pattern.captures_iter(instructions) {
        let my_instruction = &instruction[0];
        match my_instruction {
            "L" => cur_direction = cur_direction.left(),
            "R" => cur_direction = cur_direction.right(),
            number => {
                let n : u32 = number.parse().unwrap();
                for _ in 0..n {
                    cur = move_one_step(cur, cur_direction, &coords, &lines, &columns);
                }
            }
        }
    }
    let password = (cur.1 as u32 + 1) * 1000 + (cur.0 as u32 + 1) * 4 + cur_direction.score();
    println!("Password for part {part} is {password}");


}

fn move_one_step(pos: Point, direction: Direction, coords: &HashMap<Point, Tile>,
                 lines: &Vec<(i32,i32)>, columns: &Vec<(i32,i32)>) -> Point {
    let (x,y) = (pos.0 as i32, pos.1 as i32);
    let mut new_pos = match direction {
        Direction::Up => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Down => (x, y + 1),
        Direction::Right => (x + 1, y),
    };
    match coords.get(&new_pos) {
        None => {
            new_pos = match direction {
                Direction::Up => (x, columns[x as usize].1),
                Direction::Left => (lines[y as usize].1, y),
                Direction::Down => (x, columns[x as usize].0),
                Direction::Right => (lines[y as usize].0, y),
            };
            match coords.get(&new_pos) {
                None => panic!("Wrapping should always put us in existing part of the board"),
                Some(Tile::Wall) => {
                    new_pos = pos;
                },
                _ => (), // all good
            }
        },
        Some(Tile::Wall) => {
            new_pos = pos;
        },
        Some(Tile::Open) => (), // all good
    }

    new_pos
}

type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(self) -> Direction {
        self.left().left().left()
    }

    fn score(self) -> u32 {
        match self {
            Direction::Up => 3,
            Direction::Left => 2,
            Direction::Down => 1,
            Direction::Right => 0,
        }
    }
}
