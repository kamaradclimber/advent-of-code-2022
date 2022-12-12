use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();

    let mut maze: Vec<Vec<i32>> = vec![];
    let mut start = (0, 0);
    let mut stop = (0, 0);

    for (y, line) in lines.enumerate() {
        let mut maze_line = vec![];
        for (x, c) in line.chars().enumerate() {
            let height = match c {
                'S' => {
                    stop = (x as i32, y as i32);
                    0
                }
                'E' => {
                    start = (x as i32, y as i32);
                    25
                }
                _ => c as i32 - 'a' as i32,
            };
            maze_line.push(height);
        }
        maze.push(maze_line);
    }

    let mut to_explore: VecDeque<((i32, i32), i32)> = VecDeque::new();
    to_explore.push_back((start, 0));
    let mut distances = HashMap::new();
    distances.insert(start, 0);
    let mut visited = HashSet::new();

    while to_explore.len() > 0 {
        let (point, distance) = to_explore
            .pop_front()
            .expect("There is no path connecting S to E");
        if point == stop {
            break;
        }
        if part == 2 {
            let (x, y) = point;
            if maze[y as usize][x as usize] == 0 {
                stop = (x, y);
                break;
            }
        }
        if !visited.insert(point) {
            continue;
        }
        for n in neighbors(point, &maze) {
            let new_distance = distance + 1;
            let better_distance = distances
                .get(&n)
                .map_or(true, |&old_distance| old_distance > new_distance);
            if better_distance {
                distances.insert(n, new_distance);
                to_explore.push_back((n, new_distance));
            }
        }
    }

    let distance = distances
        .get(&stop)
        .expect("We escaped the loop so we have found a path");
    println!("Shortest distance from S to E is {distance}");
}

fn neighbors(point: (i32, i32), maze: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let (x, y) = point;
    let candidates: Vec<(i32, i32)> = vec![(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)];
    candidates
        .into_iter()
        .filter(|(xx, _)| *xx >= 0 && (*xx as usize) < maze[0].len())
        .filter(|(_, yy)| *yy >= 0 && (*yy as usize) < maze.len())
        .filter(|(xx, yy)| maze[*yy as usize][*xx as usize] >= maze[y as usize][x as usize] - 1)
        .collect()
}
