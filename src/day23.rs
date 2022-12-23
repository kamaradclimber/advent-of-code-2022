use std::fs;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut positions = HashSet::<Elf>::new();
    for (y, line) in lines.enumerate() {
        println!("Line is {0}", &line);
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => { positions.insert(Elf { x: x as i32, y: y as i32 }); },
                _ => (),
            }
        }
    }
    for round_id in 0..10 {
        let proposed_positions = propose_positions(&positions, round_id % 4);
        positions = effective_move(proposed_positions, &positions);
    }
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

fn propose_positions(positions: &HashSet<Elf>, round_id: usize) -> HashMap<Elf, Elf> {
    let mut proposed = HashMap::new();
    for &elf in positions {
        println!("Considering {:?}", elf);
        if elf.neighbors().iter().any(|n| positions.contains(n)) {
            // we want to move
            for d in 0..4 {
                let proposition = match (d + round_id) % 4 {
                    0 => {
                        let looked_at = [elf.nw(), elf.n(), elf.ne()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            println!("  -> Lets move north");
                            Some(elf.n())
                        } else {
                            None
                        }
                    },
                    1 => {
                        let looked_at = [elf.sw(), elf.s(), elf.se()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            println!("  -> Lets move south");
                            Some(elf.s())
                        } else { 
                            None
                        }
                    },
                    2 => {
                        let looked_at = [elf.nw(), elf.w(), elf.sw()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            println!("  -> Lets move west");
                            Some(elf.w())
                        } else {
                            None
                        }
                    },
                    3 => {
                        let looked_at = [elf.ne(), elf.e(), elf.se()];
                        if looked_at.iter().all(|p| !positions.contains(p)) {
                            println!("  -> Lets move east");
                            Some(elf.e())
                        } else {
                            None
                        }
                    },
                    _ => panic!()
                };
                if let Some(destination) = proposition {
                    println!("{0:?} proposed to move {1:?}", elf, destination);
                    proposed.insert(elf, destination);
                    break;
                }
            }
        } else {
            println!("{0:?} won't move", elf);
            proposed.insert(elf, elf);
        }
    }
    let mut real_destinations = HashMap::new();
    for (destination, elf_entries) in &proposed.iter().group_by(|(_elf, destination)| *destination) {
        let elves : Vec<Elf> = elf_entries.map(|entry| *entry.0).collect();
        println!("Considering destination: {0:?} {1}", destination, elves.len());
        if elves.len() == 1 {
            real_destinations.insert(elves[0], *destination);
        } else {
            for elf in elves {
                println!("{0:?} wont move after all", elf);
                real_destinations.insert(elf, elf);
            }
        }
    }
    assert_eq!(real_destinations.len(), positions.len());
    real_destinations
}

fn effective_move(proposed_positions: HashMap<Elf,Elf>, positions: &HashSet<Elf>) -> HashSet<Elf> {
    todo!();
}
