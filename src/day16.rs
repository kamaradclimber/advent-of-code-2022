use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut valve_ids : HashMap<String, usize> = HashMap::new();
    let mut tunnels : Vec<Vec<usize>> = vec![]; // valve id -> valve id
    let mut valve_flow_rates : Vec<u32> = vec![];
    let re = Regex::new(r"^Valve (.+) has flow rate=(\d+); tunnel[^ ]* lead[^ ]* to valve[^ ]* (.+)$").unwrap();
    for line in lines {
        println!("Line is {0}", &line);
        let cap = re.captures(line).unwrap();
        let valve = String::from(&cap[1]);
        let flow_rate = cap[2].parse::<u32>().unwrap();
        let id = *valve_ids.entry(valve).or_insert(valve_flow_rates.len());
        valve_flow_rates.insert(id, flow_rate);
        tunnels.insert(id, vec![]);
    }
    for line in contents.lines() {
        println!("Line is {0}", &line);
        let cap = re.captures(line).unwrap();
        let valve = String::from(&cap[1]);
        let dests = cap[3].split(", ");
        let id = *valve_ids.entry(valve).or_insert(valve_flow_rates.len());
        for dest in dests {
            let dest_id = *valve_ids.get(&String::from(dest)).unwrap();
            tunnels[id].push(dest_id);
        }
    }
    dbg!(&valve_ids);
    let res = maximum_flow_released(*valve_ids.get("AA").unwrap(), &valve_flow_rates, &tunnels);
    println!("Solution for {part} is {res}");
}

type ValveId = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Visited {
    state : u32,
}
impl std::fmt::Debug for Visited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut id = 0;
        let base : u32 = 2;
        while self.state >= base.pow(id) {
            let xbit = base.pow(id);
            let c = if xbit & self.state == xbit { 1 } else { 0 };
            write!(f, "{c}")?;
            id +=1;
        }
        Ok(())
    }
}

impl Visited {
    fn is_activated(self, valve_id: ValveId) -> bool {
        let base : u32 = 2;
        let xbit = base.pow(valve_id as u32);
        self.state & xbit == xbit
    }

    fn visit(self, valve_id: ValveId) -> Visited {
        let base : u32 = 2;
        let xbit = base.pow(valve_id as u32);
        Visited { state: self.state | xbit }
    }
}

#[derive(Clone, Debug)]
enum Path<'a> {
    Empty,
    NonEmpty(ValveId, &'a Path<'a>)
}

fn all_paths(distances: &Vec<Vec<Option<u32>>>, time_budget: u32, start_point: ValveId) -> Vec<Path> {
    let _all_paths = |time_budget: u32, visited: Visited, current_point: ValveId, path: Path| {
        todo!()
    };
    _all_paths(30, Visited { state: 0 }, start_point, Path::Empty)
}

fn build_distances(tunnels: &Vec<Vec<ValveId>>) -> Vec<Vec<Option<u32>>> {
    let mut known_distances = vec![vec![None; tunnels.len()]; tunnels.len()];
    for start in 0..tunnels.len() {
        let mut to_explore: VecDeque<(ValveId, u32)> = VecDeque::new();
        to_explore.push_back((start, 0));
        while to_explore.len() > 0 {
            let (tunnel, distance) = to_explore.pop_front().unwrap();
            for neighbor_tunnel in &tunnels[tunnel] {
                if *neighbor_tunnel == start {
                    // it's important not to fill known_distances[x][x] with something to avoid infinite path
                    continue;
                }
                if known_distances[start][*neighbor_tunnel].is_some() {
                    continue; // we already know shortest path
                }
                known_distances[start][*neighbor_tunnel] = Some(distance + 1);
                known_distances[*neighbor_tunnel][start] = Some(distance + 1);
                to_explore.push_back((*neighbor_tunnel, distance+1));
            }

        }
    }
    known_distances
}

fn maximum_flow_released(start_id: ValveId, valve_flow_rates: &Vec<u32>, tunnels: &Vec<Vec<ValveId>>) -> u32 {
    println!("Step 1: compute distances between all nodes");
    let distances = build_distances(tunnels);

    // some verifications first
    //dbg!(&distances[0]);

    println!("Step 2: build a smaller graph with only valves that have a >0 flow");
    let mut simpler_flow_rates : Vec<u32> = vec![];
    let mut corresponding_table : HashMap<ValveId, ValveId> = HashMap::new();
    corresponding_table.insert(0, start_id); // let's keep the AA tunnel
    simpler_flow_rates.insert(0, valve_flow_rates[start_id]);

    for (valve_id, flow) in valve_flow_rates.iter().enumerate() {
        if *flow > 0 {
            let new_valve_id = simpler_flow_rates.len();
            corresponding_table.insert(new_valve_id, valve_id);
            simpler_flow_rates.push(*flow);
        }
    }
    let mut simpler_distances = vec![vec![None; simpler_flow_rates.len()]; simpler_flow_rates.len()];
    for start_new_id in 0..simpler_distances.len() {
        for end_new_id in 0..simpler_distances.len() {
            let start_old_id = corresponding_table.get(&start_new_id).unwrap();
            let end_old_id = corresponding_table.get(&end_new_id).unwrap();
            simpler_distances[start_new_id][end_new_id] = distances[*start_old_id][*end_old_id];
        }
    }

    dbg!(&corresponding_table);
    dbg!(&simpler_flow_rates);

    println!("Find better path");
    let mut max = 0;
    for path in all_paths(&simpler_distances, 30, start_id) {
        max = std::cmp::max(compute_total_pressure(&path, &simpler_distances, start_id, &simpler_flow_rates).0, max);
    }
    max
}

fn compute_total_pressure(path: &Path, distances: &Vec<Vec<Option<u32>>>, current_pos: ValveId, valve_flows: &Vec<u32>) -> (u32, u32, Visited) { // (total_pressure, current_time, already activated)
    match path {
        Path::Empty => (0, 0, Visited { state: 0 }),
        Path::NonEmpty(dest, tail) => {
            let (total_pressure, current_time, activated) = compute_total_pressure(tail, distances, *dest, valve_flows);
            todo!()
        }
    }
}
