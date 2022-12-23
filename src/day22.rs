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

    if part == 1 {
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
    } else { // part 2
        let face_area = (coords.iter().len() / 6) as u32;
        let edge_size = (1..100).find(|i| i * i == face_area ).expect("We support cube of edge size 100 maximum");
        let _mappings = folding(edge_size, cur, cur_direction, &coords);
    }
    let password = (cur.1 as u32 + 1) * 1000 + (cur.0 as u32 + 1) * 4 + cur_direction.score();
    println!("Password for part {part} is {password}");


}

fn print(coords: &HashMap<Point, Tile>, edge_size: u32, point: Point, dir: Direction) {
    for y in 0..=(edge_size*5) {
        for x in 0..=(edge_size*5) {
            let p = (x as i32, y as i32);
            if p == point {
                match dir { 
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                }
            } else {
                match coords.get(&p) {
                    None => print!(" "),
                    Some(Tile::Wall) => print!("#"),
                    _ => print!("."),
                }
            }
        }
        println!("");
    }
}

#[derive(Debug, Clone, Copy)]
enum LastTurnType {
    Right,
    Left,
    Straight
}

fn folding(edge_size: u32, initial_point: Point, initial_direction: Direction, coords: &HashMap<Point,Tile>) -> HashMap<Point, Point> {
    let mut edge_stack : Vec<(Point, Direction)> = vec![];
    let mut folding = HashMap::new();
    let mut cur_pos = (initial_point.0-1, initial_point.1);
    let mut cur_dir = initial_direction;
    let mut last_turn_type = None;
    edge_stack.push((initial_point, initial_direction));
    loop {
        print(&coords, edge_size, cur_pos, cur_dir);
        println!("  at {0},{1} (0-indexed), dir {cur_dir:?}", cur_pos.0, cur_pos.1);
        if last_turn_type.is_some() && cur_pos == initial_point {
            break;
        }
        //let my_ahead = next_pos(cur_pos, cur_dir, edge_size+1);
        let my_ahead = next_pos(cur_pos, cur_dir, 1);
        let my_left = next_pos(my_ahead, cur_dir.left(), 1);
        match (coords.get(&my_ahead), coords.get(&my_left)) {
            (None, None) => {
                cur_pos = next_pos(cur_pos, cur_dir.left().left(), 1);
                cur_pos = next_pos(cur_pos, cur_dir, 1);
                // change our direction
                cur_dir = cur_dir.right();
                // record last turn
                last_turn_type = Some(LastTurnType::Right);
                println!("== Turn right");
                edge_stack.push((cur_pos, cur_dir));
            },
            (None, Some(_)) => panic!("I am not sure this case can/should happen"),
            (Some(_), Some(_)) => {
                cur_pos = next_pos(cur_pos, cur_dir, 1);
                // change our direction
                cur_dir = cur_dir.left();
                last_turn_type = Some(LastTurnType::Left);
                println!("== Turn left");
                edge_stack.push((cur_pos, cur_dir));
            },
            (Some(_), None) => {
                cur_pos = next_pos(cur_pos, cur_dir, 1);
                last_turn_type = Some(LastTurnType::Straight);
                println!("== Continue straight ahead");
            },
        }
        match last_turn_type {
            None => (),
            Some(LastTurnType::Left) => {
                let (p1, dir1) = edge_stack.pop().unwrap();
                let (p2, dir2) = edge_stack.pop().unwrap();
                for step in 1..edge_size {
                    let a= next_pos(p1, dir1, step);
                    let b = next_pos(p2, dir2.left().left(), edge_size - step);
                    println!("{0:?} <-> {1:?}", a, b);
                    folding.insert(a,b);
                    folding.insert(b,a);
                }
            },
            _ => (), // nothing for now
        }
    }

    folding
}

fn move_wrap(pos: Point, direction: Direction, lines: &Vec<(i32,i32)>, columns: &Vec<(i32,i32)>) -> Point {
    let (x,y) = (pos.0 as i32, pos.1 as i32);
    match direction {
        Direction::Up => (x, columns[x as usize].1),
        Direction::Left => (lines[y as usize].1, y),
        Direction::Down => (x, columns[x as usize].0),
        Direction::Right => (lines[y as usize].0, y),
    }
}

// return a point that might not be on the board
fn next_pos(pos: Point, direction: Direction, step_size: u32) -> Point {
    let (mut x,mut y) = (pos.0 as i32, pos.1 as i32);
    for _ in 0..step_size {
        (x, y) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
        }
    }
    (x,y)
}

fn move_one_step(pos: Point, direction: Direction, coords: &HashMap<Point, Tile>,
                 lines: &Vec<(i32,i32)>, columns: &Vec<(i32,i32)>) -> Point {
    // get a candidate new pos
    let mut new_pos = next_pos(pos, direction, 1);
    // wrap it, undo it or keep it
    match coords.get(&new_pos) {
        None => { // wrap
            new_pos = move_wrap(pos, direction, lines, columns);
            match coords.get(&new_pos) {
                None => panic!("Wrapping should always put us in existing part of the board"),
                Some(Tile::Wall) => {
                    new_pos = pos;
                },
                _ => (), // all good
            }
        },
        Some(Tile::Wall) => { // undo
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
