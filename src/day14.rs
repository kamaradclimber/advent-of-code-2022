use std::cmp::{max, min};
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut rock_lines = read_rock_lines(lines);

    // let's cheat a little bit to make sure (500, 0) is included
    rock_lines.push((Point { x: 501, y: 0 }, Point { x: 501, y: 0 }));

    let (top_left, bottom_right) = bounding_coordinates(&rock_lines);
    if part == 2 {
        dbg!(bottom_right);
        // let's add a "infinite" line at the bottom
        let shift = bottom_right.y + 2;
        rock_lines.push((
            Point {
                x: top_left.x - shift,
                y: bottom_right.y + 2,
            },
            Point {
                x: bottom_right.x + shift,
                y: bottom_right.y + 2,
            },
        ));
    }
    let (top_left, bottom_right) = bounding_coordinates(&rock_lines);

    // shifting coordinates
    let rock_lines = rock_lines.iter().map(|(p1, p2)| {
        let p1 = Point {
            x: p1.x - top_left.x,
            y: p1.y - top_left.y,
        };
        let p2 = Point {
            x: p2.x - top_left.x,
            y: p2.y - top_left.y,
        };
        (p1, p2)
    });
    let starting_sand_point = Point {
        x: 500 - top_left.x,
        y: 0 - top_left.y,
    };

    let width = bottom_right.x - top_left.x + 1;
    let height = bottom_right.y - top_left.y + 1;
    let mut cave = vec![vec![Item::Void; width]; height];
    for (p1, p2) in rock_lines {
        for p in p1.straight_line_to(&p2) {
            cave[p.y][p.x] = Item::Rock;
        }
    }
    cave[starting_sand_point.y][starting_sand_point.x] = Item::StartingSandPoint;

    let mut sand_unit_count = 1;
    loop {
        let sand_point = Point { ..starting_sand_point };
        // now this sand will fall as low as possible
        match fall(sand_point, &cave) {
            None => break,
            Some(still_point) => {
                if matches!(cave[still_point.y][still_point.x], Item::StartingSandPoint) {
                    sand_unit_count += 1;
                    break;
                }
                cave[still_point.y][still_point.x] = Item::Sand
            }
        }
        sand_unit_count += 1;
    }
    let my_cave = Cave { coords: &cave };
    dbg!(&my_cave);
    println!("Solution for {part} is {0}", sand_unit_count - 1);
}

// return the still position after falling or None if point will be falling for eternity
fn fall(sand_point: Point, cave: &Vec<Vec<Item>>) -> Option<Point> {
    let mut current_pos = sand_point;
    loop {
        if current_pos.y + 1 >= cave.len() {
            return None;
        }
        let line_below = &cave[current_pos.y + 1];
        let below = line_below[current_pos.x];
        let below_left = line_below[current_pos.x - 1];
        let below_right = line_below[current_pos.x + 1];
        match (below, below_left, below_right) {
            (Item::Void, _, _) => {
                current_pos = Point {
                    y: current_pos.y + 1,
                    x: current_pos.x,
                }
            }
            (_, Item::Void, _) => {
                current_pos = Point {
                    y: current_pos.y + 1,
                    x: current_pos.x - 1,
                }
            }
            (_, _, Item::Void) => {
                current_pos = Point {
                    y: current_pos.y + 1,
                    x: current_pos.x + 1,
                }
            }
            _ => return Some(current_pos),
        }
    }
}

struct Cave<'a> {
    coords: &'a Vec<Vec<Item>>,
}

impl std::fmt::Debug for Cave<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "\n")?;
        for line in self.coords {
            for item in line {
                write!(f, "{:?}", item)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Item {
    Void,
    Rock,
    Sand,
    StartingSandPoint,
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Item::Void => write!(f, ".")?,
            Item::Rock => write!(f, "#")?,
            Item::Sand => write!(f, "o")?,
            Item::StartingSandPoint => write!(f, "+")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn straight_line_to(&self, other: &Point) -> Vec<Point> {
        let mut line = vec![];
        if other.x == self.x {
            for y in min(other.y, self.y)..=max(other.y, self.y) {
                line.push(Point { x: self.x, y });
            }
        } else {
            for x in min(other.x, self.x)..=max(other.x, self.x) {
                line.push(Point { x, y: self.y });
            }
        }
        line
    }
}

fn bounding_coordinates(rock_lines: &Vec<(Point, Point)>) -> (Point, Point) {
    let mut all_points = rock_lines.iter().flat_map(|(p1, p2)| vec![p1, p2]);
    let first_point: Point = *all_points.next().expect("Need at least one point");
    let mut top_left = first_point;
    let mut bottom_right = first_point;
    for point in all_points {
        if point.x > bottom_right.x {
            bottom_right = Point { x: point.x, ..bottom_right };
        }
        if point.x < top_left.x {
            top_left = Point { x: point.x, ..top_left };
        }
        if point.y < top_left.y {
            top_left = Point { y: point.y, ..top_left };
        }
        if point.y > bottom_right.y {
            bottom_right = Point { y: point.y, ..bottom_right }
        }
    }
    // having a security margin will allow to manipulate Point at the edge of the map without
    // too many difficulty, specifically it allows to consider points bottom, bottom left and
    // bottom right in all situations (even if those points don't exist)
    let x_safety_margin = 1;
    (
        Point {
            x: top_left.x - x_safety_margin,
            ..top_left
        },
        Point {
            x: bottom_right.x + x_safety_margin,
            ..bottom_right
        },
    )
}

fn read_rock_lines(lines: std::str::Lines) -> Vec<(Point, Point)> {
    let mut rock_lines = vec![];
    for line in lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        for rock_line in points.windows(2) {
            let start = read_point(rock_line[0]);
            let end = read_point(rock_line[1]);
            rock_lines.push((start, end));
        }
    }
    rock_lines
}

fn read_point(point: &str) -> Point {
    let mut xy_as_string = point.split(",");
    let x = xy_as_string.next().unwrap().parse().expect("x coordinate should contain a valid integer");
    let y = xy_as_string.next().unwrap().parse().expect("y coordinate should contain a valid integer");
    Point { x, y }
}
