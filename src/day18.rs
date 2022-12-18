use std::collections::{HashSet, VecDeque};
use std::fs;
use std::ops::RangeInclusive;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut droplets = HashSet::new();
    for line in lines {
        let droplet: Point = line
            .parse()
            .expect("Input should be valid but this point cannot be parsed");
        droplets.insert(droplet);
    }

    if part == 1 {
        let mut surface_area = 0;
        for droplet in &droplets {
            let neighbors = droplet.neighbors();
            surface_area += 6 - neighbors.iter().filter(|n| droplets.contains(&n)).count();
        }
        println!("Solution for part {part} is {surface_area}");
    } else {
        let min_x = droplets.iter().map(|d| d.x).min().unwrap() - 1;
        let min_y = droplets.iter().map(|d| d.y).min().unwrap() - 1;
        let min_z = droplets.iter().map(|d| d.z).min().unwrap() - 1;
        let max_x = droplets.iter().map(|d| d.x).max().unwrap() + 1;
        let max_y = droplets.iter().map(|d| d.y).max().unwrap() + 1;
        let max_z = droplets.iter().map(|d| d.z).max().unwrap() + 1;

        // let mut ambiant_air = vec![];
        let mut exterior: HashSet<Point> = std::iter::empty()
            .chain((min_y..=max_y).flat_map(|y| {
                (min_z..=max_z)
                    .clone()
                    .map(move |z| Point { x: min_x, y, z })
            }))
            .chain((min_y..=max_y).flat_map(|y| {
                (min_z..=max_z)
                    .clone()
                    .map(move |z| Point { x: max_x, y, z })
            }))
            .chain((min_x..=max_x).flat_map(|x| {
                (min_z..=max_z)
                    .clone()
                    .map(move |z| Point { x, y: min_y, z })
            }))
            .chain((min_x..=max_x).flat_map(|x| {
                (min_z..=max_z)
                    .clone()
                    .map(move |z| Point { x, y: max_y, z })
            }))
            .chain((min_x..=max_x).flat_map(|x| {
                (min_y..=max_y)
                    .clone()
                    .map(move |y| Point { x, y, z: min_z })
            }))
            .chain((min_x..=max_x).flat_map(|x| {
                (min_y..=max_y)
                    .clone()
                    .map(move |y| Point { x, y, z: max_z })
            }))
            .filter(|p| !droplets.contains(&p))
            .collect();

        let mut to_explore: VecDeque<Point> = VecDeque::new();
        for p in &exterior {
            to_explore.push_back(*p);
        }
        while to_explore.len() > 0 {
            let considered_point = to_explore.pop_front().unwrap();
            if !droplets.contains(&considered_point) {
                exterior.insert(considered_point);
                let new_to_explore = considered_point
                    .neighbors()
                    .into_iter()
                    .filter(|n| n.within(min_x..=max_x, min_y..=max_y, min_z..=max_z))
                    .filter(|n| !exterior.contains(&n));
                for p in new_to_explore {
                    to_explore.push_back(p);
                }
            }
        }
        let mut exterior_surface_area = 0;
        for droplet in &droplets {
            let neighbors = droplet.neighbors();
            exterior_surface_area += neighbors.iter().filter(|n| exterior.contains(&n)).count();
        }
        println!(
            "Number of cubes total within the area is {0}",
            (max_x - min_x + 1) * (max_y - min_y + 1) * (max_z - min_z + 1)
        );
        println!("Number of droplets is {0}", droplets.len());
        println!("Number of cubes in the air is {0}", exterior.len());
        println!("Solution for {part} is {exterior_surface_area}");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn within(
        self,
        x_range: RangeInclusive<i32>,
        y_range: RangeInclusive<i32>,
        z_range: RangeInclusive<i32>,
    ) -> bool {
        x_range.contains(&self.x) && y_range.contains(&self.y) && z_range.contains(&self.z)
    }

    fn neighbors(self) -> Vec<Point> {
        let mut my_neighbors = vec![];
        my_neighbors.push(Point {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        });
        my_neighbors.push(Point {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        });
        my_neighbors.push(Point {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        });
        my_neighbors.push(Point {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        });
        my_neighbors.push(Point {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        });
        my_neighbors.push(Point {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        });
        my_neighbors
    }
}

impl std::str::FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(",");
        if let (Some(x), Some(y), Some(z)) = (coords.next(), coords.next(), coords.next()) {
            Ok(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            })
        } else {
            Err("Invalid point, not enough coordinates")
        }
    }
}
