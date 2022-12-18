use std::collections::HashSet;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut droplets = HashSet::new();
    for line in lines {
        let droplet: LavaDroplet = line
            .parse()
            .expect("Input should be valid but this point cannot be parsed");
        droplets.insert(droplet);
    }

    let mut surface_area = 0;
    for droplet in &droplets {
        let neighbors = droplet.neighbors();
        surface_area += 6 - neighbors.iter().filter(|n| droplets.contains(&n)).count();
    }
    println!("Solution for part {part} is {surface_area}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LavaDroplet {
    x: i32,
    y: i32,
    z: i32,
}

impl LavaDroplet {
    fn neighbors(self) -> Vec<LavaDroplet> {
        let mut my_neighbors = vec![];
        my_neighbors.push(LavaDroplet {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        });
        my_neighbors.push(LavaDroplet {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        });
        my_neighbors.push(LavaDroplet {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        });
        my_neighbors.push(LavaDroplet {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        });
        my_neighbors.push(LavaDroplet {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        });
        my_neighbors.push(LavaDroplet {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        });
        my_neighbors
    }
}

impl std::str::FromStr for LavaDroplet {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(",");
        if let (Some(x), Some(y), Some(z)) = (coords.next(), coords.next(), coords.next()) {
            Ok(LavaDroplet {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            })
        } else {
            Err("Invalid point, not enough coordinates")
        }
    }
}
