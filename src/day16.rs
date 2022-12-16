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

fn dfs_compute(current_id: ValveId, total_flow_released_so_far: u32, flow_releasing_so_far: u32 ,remaining_time: u32, simpler_distance: &Vec<Vec<Option<u32>>>, simpler_flow_rates: &Vec<u32>, activated_already: Visited) -> u32 {
    if remaining_time == 0 {
        return total_flow_released_so_far;
    }
    let base : u32 = 2;
    let all_visited = Visited { state: base.pow(simpler_flow_rates.len() as u32) - 1 };
    if activated_already == all_visited {
        return total_flow_released_so_far;
    }
    let mut max_total_flow = 0;
    for (dest_id, opt_time) in simpler_distance[current_id].iter().enumerate() {
        match opt_time {
            None => continue,
            Some(time) if time > &remaining_time => continue,
            Some(time) => {
                let total_flow_released_so_far = total_flow_released_so_far + flow_releasing_so_far;
                let (time_spent, flow_releasing_so_far, activated_already) = if activated_already.is_activated(dest_id) || time + 1 > remaining_time{
                    println!("Just visiting {dest_id}, releasing {flow_releasing_so_far}");
                    (*time, flow_releasing_so_far, activated_already)
                } else {
                    println!("Activating {dest_id}, releasing {flow_releasing_so_far}/min. {activated_already:?}");
                    let new_flow = flow_releasing_so_far + simpler_flow_rates[dest_id];
                    (time+1, new_flow, activated_already.visit(dest_id))
                };
                println!("Remaining time {0}", remaining_time - time_spent);
                let res = dfs_compute(dest_id, total_flow_released_so_far, flow_releasing_so_far, remaining_time - time_spent, simpler_distance, simpler_flow_rates, activated_already);
                max_total_flow = std::cmp::max(max_total_flow, res);
            }
        }
    }
    return total_flow_released_so_far;
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
    dfs_compute(start_id, 0, 0, 30, &simpler_distances, &simpler_flow_rates, Visited { state: 0 })
}

