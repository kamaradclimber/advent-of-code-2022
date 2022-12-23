use std::fs;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut positions = HashSet::<Elf>::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { positions.insert(Elf { x: x as i32, y: y as i32 }); },
                _ => (),
            }
        }
    }
    for round_id in 0..10 {
        positions = propose_positions(&positions, round_id % 4);
    }
    let (minx,maxx) = positions.iter().map(|elf| elf.x).minmax().into_option().unwrap();
    let (miny,maxy) = positions.iter().map(|elf| elf.y).minmax().into_option().unwrap();
    let empty_tiles = (maxx-minx+1) * (maxy-miny+1) - positions.len() as i32;
    println!("Solution for {part} is {empty_tiles}");
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Elf {
    x: i32,
    y: i32
}

impl std::fmt::Debug for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({0},{1})", self.x, self.y)
    }
}

impl Elf {
    fn neighbors(&self) -> [Elf; 8]{
        [self.nw(), self.n(), self.ne(), self.w(), self.e(), self.sw(), self.s(), self.se()]
    }

    fn nw(&self) -> Elf { Elf { x: self.x-1, y: self.y-1 } }
    fn n(&self) -> Elf { Elf { x: self.x, y: self.y-1 } }
    fn ne(&self) -> Elf { Elf { x: self.x+1, y: self.y-1 } }
    fn w(&self) -> Elf { Elf { x: self.x-1, y: self.y } }
    fn e(&self) -> Elf { Elf { x: self.x+1, y: self.y } }
    fn sw(&self) -> Elf { Elf { x: self.x-1, y: self.y+1 } }
    fn s(&self) -> Elf { Elf { x: self.x, y: self.y+1 } }
    fn se(&self) -> Elf { Elf { x: self.x+1, y: self.y+1 } }
}

fn propose_positions(positions: &HashSet<Elf>, round_id: usize) -> HashSet<Elf> {
    let mut proposed = HashMap::new();
    for &elf in positions {
        if elf.neighbors().iter().any(|n| positions.contains(n)) {
            // we want to move
            for d in 0..4 {
                let proposition = match (d + round_id) % 4 {
                    0 => {
                        let looked_at = [elf.nw(), elf.n(), elf.ne()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            Some(elf.n())
                        } else {
                            None
                        }
                    },
                    1 => {
                        let looked_at = [elf.sw(), elf.s(), elf.se()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            Some(elf.s())
                        } else { 
                            None
                        }
                    },
                    2 => {
                        let looked_at = [elf.nw(), elf.w(), elf.sw()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            Some(elf.w())
                        } else {
                            None
                        }
                    },
                    3 => {
                        let looked_at = [elf.ne(), elf.e(), elf.se()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            Some(elf.e())
                        } else {
                            None
                        }
                    },
                    _ => panic!()
                };
                if let Some(destination) = proposition {
                    proposed.insert(elf, destination);
                    break;
                }
            }
            if !proposed.contains_key(&elf) {
                // cant move anywhere
                proposed.insert(elf, elf);
            }
        } else {
            proposed.insert(elf, elf);
        }
    }
    let mut real_destinations = HashSet::new();
    let mut groups = HashMap::new();
    for (elf, dest) in proposed.iter() {
        groups.entry(*dest).and_modify(|l : &mut Vec<Elf>| l.push(*elf)).or_insert(vec![*elf]);
    }
    for (destination, elves) in groups {
        if elves.len() == 1 {
            real_destinations.insert(destination);
        } else {
            for elf in elves {
                real_destinations.insert(elf);
            }
        }
    }
    assert_eq!(real_destinations.len(), positions.len());
    real_destinations
}
