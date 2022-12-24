use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut valve_name_to_id = HashMap::<String, Location>::new();
    let mut neighbors : HashMap<Location, Vec<Location>> = HashMap::new();
    let mut flows : Vec<u32> = vec![];
    let flow_regex = Regex::new(r"Valve (.+) has flow rate=(\d+);").unwrap();
    for line in lines {
        let cap = flow_regex.captures(line).expect("Input should be valid");

        let valve_name = cap[1].to_string();
        let flow : u32 = cap[2].parse().unwrap();
        valve_name_to_id.insert(valve_name, flows.len() as u8);
        flows.push(flow);
    }
    let tunnel_regex = Regex::new(r"([A-Z][A-Z]+)").unwrap();
    for line in contents.lines() {
        let mut matches = tunnel_regex.captures_iter(line);
        let source = matches.next().unwrap()[1].to_string(); 
        let source_id = *valve_name_to_id.get(&source).unwrap();
        while let Some(m) = matches.next() {
            let dest_id = *valve_name_to_id.get(&m[1].to_string()).unwrap();
            neighbors.entry(source_id).and_modify(|dests| dests.push(dest_id)).or_insert(vec![dest_id]);
            neighbors.entry(dest_id).and_modify(|dests| dests.push(source_id)).or_insert(vec![source_id]);
        }
    }

    let mut best_pressure = None;
    let mut tried = HashSet::<(Time, Location, BitSet, ReleasedPressure)>::new();
    let mut to_explore = vec![(30, 0, BitSet::empty(), 0)];

    while to_explore.len() > 0 {
        let (time, location, opened_valves, total_pressure) = to_explore.pop().unwrap();
        if time == 0 {
            match best_pressure {
                None => best_pressure = Some(total_pressure),
                Some(p) => {
                    if total_pressure > p {
                        // println!("New best pressure {0}", best_pressure.unwrap());
                        best_pressure = Some(total_pressure);
                    }
                },
            };
            continue;
        }
        if tried.contains(&(time, location, opened_valves, total_pressure)) {
            continue;
        }

        {
            let mut max = 0;
            for closed_valve in opened_valves.still_closed((flows.len() - 1) as u8) {
                max += flows[closed_valve as usize] * (time-1);
            }
            if best_pressure.is_some() {
                if total_pressure + max <= best_pressure.unwrap() {
                    // println!("Even opening all remaining valves simultanously now would not be better");
                    continue;
                }
            }
        }

        // option1: we could move to another location
        for n in neighbors[&location].iter() {
            if opened_valves.is_opened(*n) {
                to_explore.push((time-1, *n, opened_valves, total_pressure));
            }
        }
        for n in neighbors[&location].iter() {
            if !opened_valves.is_opened(*n) {
                to_explore.push((time-1, *n, opened_valves, total_pressure));
            }
        }
        // option2: we open the current valve if possible
        if flows[location as usize] > 0 {
            if !opened_valves.is_opened(location) {
                let total_pressure = total_pressure + flows[location as usize] * (time - 1);
                to_explore.push((time-1, location, opened_valves.open(location), total_pressure));
            }
        }
        tried.insert((time, location, opened_valves, total_pressure));
    }
    println!("Solution for {part} is {0}", best_pressure.unwrap());
}

type Time = u32;
type Location = u8;
type ReleasedPressure = u32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct BitSet {
    repr: u64
}

impl BitSet {
    fn empty() -> BitSet {
        BitSet { repr : 0 }
    }

    fn is_opened(&self, valve_id: u8) -> bool {
        let base = 2 as u64;
        base.pow(valve_id.into()) & self.repr == base.pow(valve_id.into())
    }

    fn open(&self, valve_id: u8) -> BitSet {
        let base = 2 as u64;
        BitSet { repr: base.pow(valve_id.into()) | self.repr }

    }

    fn still_closed(&self, max_id: u8) -> Vec<u8> {
        let mut closed = vec![];
        for i in 0..=max_id {
            if !self.is_opened(i) {
                closed.push(i);
            }
        }
        closed
    }
}
