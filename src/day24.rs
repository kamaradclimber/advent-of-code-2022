use std::collections::{HashMap, HashSet};
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();

    let mut valley: ValleyState = ValleyState::new();
    let height = contents.lines().into_iter().count();
    let width = contents.lines().next().unwrap().len();
    for (y, line) in lines.enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if chars[1] == '#' || chars[chars.len() - 2] == '#' {
            continue;
        }
        for (x, c) in chars.iter().enumerate() {
            let p = Point { x, y };
            match c {
                '^' => {
                    valley.insert(p, vec![Blizzard::Up]);
                }
                'v' => {
                    valley.insert(p, vec![Blizzard::Down]);
                }
                '<' => {
                    valley.insert(p, vec![Blizzard::Left]);
                }
                '>' => {
                    valley.insert(p, vec![Blizzard::Right]);
                }
                _ => (),
            }
        }
    }
    let mut valleys = vec![];
    valleys.push(valley);
    let turn_to_simulate = 200;
    for _ in 0..turn_to_simulate {
        let last = &valleys[valleys.len() - 1];
        // print(last, height, width);
        valleys.push(tick(last, height, width));
    }

    // now let's do a DFS to explore the valley
    let mut to_explore: Vec<(Trip, Time, Point)> = vec![];
    let valley_start = Point { x: 1, y: 0 };
    to_explore.push((0, 0, valley_start));
    let mut best_time: Option<Time> = None;
    // vvvv This could be an optimization to gain some time but it is a bit cheating
    best_time = Some(1500); // just some optim to avoid looking uselessly
    let valley_exit = Point { x: width - 2, y: height - 1 };
    let mut already_explored = HashSet::<(Trip, Time, Point)>::new();

    let mut objectives = vec![];
    objectives.push(valley_exit);
    if part == 2 {
        objectives.push(valley_start);
        objectives.push(valley_exit);
    }

    while to_explore.len() > 0 {
        let (mut trip, time, current_pos) = to_explore.pop().expect("we just checked there was an element");
        let current_objective = objectives[trip];
        if current_pos == current_objective {
            if trip == objectives.len() - 1 {
                best_time = match best_time {
                    None => Some(time),
                    Some(t) => Some(std::cmp::min(t, time)),
                };
                // println!("Best known time is {0}", best_time.unwrap());
                continue;
            } else {
                trip += 1;
            }
        }
        if already_explored.contains(&(trip, time, current_pos)) {
            continue;
        }
        if best_time.is_some() {
            if time >= best_time.unwrap() {
                continue;
            }
            let min_remaining_time = diff(current_objective.y, current_pos.y) + diff(current_objective.x, current_pos.x);
            if time + min_remaining_time >= best_time.unwrap() {
                continue;
            }
        }
        if time + 1 >= valleys.len() {
            // we need to simulate the future a bit more
            let last = &valleys[valleys.len() - 1];
            valleys.push(tick(last, height, width));
        }
        let future_valley_state = &valleys[time + 1];
        let possible_places = current_pos.neighbors(future_valley_state, height, width);
        for place in possible_places {
            to_explore.push((trip, time + 1, place));
        }
        already_explored.insert((trip, time, current_pos));
    }
    println!("Best known time for {part} is {0}", best_time.unwrap());
}

type Trip = usize;
type Time = usize;
type ValleyState = HashMap<Point, Vec<Blizzard>>;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}

impl Blizzard {
    fn next(&self, p: &Point, height: usize, width: usize) -> Point {
        let candidate = match self {
            Blizzard::Up => Point { y: p.y - 1, ..*p },
            Blizzard::Down => Point { y: p.y + 1, ..*p },
            Blizzard::Left => Point { x: p.x - 1, ..*p },
            Blizzard::Right => Point { x: p.x + 1, ..*p },
        };
        if candidate.x == 0 {
            Point { x: width - 2, ..candidate }
        } else if candidate.x == width - 1 {
            Point { x: 1, ..candidate }
        } else if candidate.y == 0 {
            Point { y: height - 2, ..candidate }
        } else if candidate.y == height - 1 {
            Point { y: 1, ..candidate }
        } else {
            candidate
        }
    }
}

fn print(valley: &ValleyState, height: usize, width: usize) {
    for i in 0..width {
        if i == 1 {
            print!(".");
        } else {
            print!("#");
        }
    }
    println!("");
    for y in 1..height - 1 {
        print!("#");
        for x in 1..width - 1 {
            match valley.get(&Point { x, y }) {
                None => print!("."),
                Some(local_blizzards) => match local_blizzards.len() {
                    x if x >= 10 => print!("µ"),
                    1 => match local_blizzards[0] {
                        Blizzard::Up => print!("^"),
                        Blizzard::Down => print!("v"),
                        Blizzard::Left => print!("<"),
                        Blizzard::Right => print!(">"),
                    },
                    x => print!("{}", x),
                },
            }
        }
        println!("#");
    }
    for i in 0..width {
        if i == width - 2 {
            print!(".");
        } else {
            print!("#");
        }
    }
    println!("");
}

fn tick(valley: &ValleyState, height: usize, width: usize) -> ValleyState {
    let mut new_valley = ValleyState::new();
    for (p, blizzards) in valley {
        for &b in blizzards {
            let new_position = b.next(p, height, width);
            new_valley.entry(new_position).and_modify(|bs| bs.push(b)).or_insert(vec![b]);
        }
    }
    new_valley
}

impl Point {
    fn valid(self, height: usize, width: usize) -> bool {
        let target = Point { x: width - 2, y: height - 1 };
        let origin = Point { x: 1, y: 0 };
        self == origin || self == target || (self.y > 0 && self.y < height - 1 && self.x > 0 && self.x < width - 1)
    }

    fn neighbors(self, future_valley_state: &ValleyState, height: usize, width: usize) -> Vec<Point> {
        let mut res = vec![];
        res.push(self);
        if self.y > 0 {
            // this check covers the initial case
            res.push(Point { y: self.y - 1, ..self });
        }
        res.push(Point { x: self.x - 1, ..self });
        res.push(Point { y: self.y + 1, ..self });
        res.push(Point { x: self.x + 1, ..self });

        let mut my_res = vec![];
        for p in res {
            if p.valid(height, width) && !future_valley_state.contains_key(&p) {
                my_res.push(p);
            }
        }
        my_res
    }
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
