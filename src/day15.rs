use std::fs;
use regex::Regex;
use std::collections::{HashMap,HashSet};

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let considered_line = 10;
    println!("Considered line is **{considered_line}**");
    let lines = contents.lines();
    let mut readings = HashMap::new();
    let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    for line in lines {
        let cap = re.captures(line).unwrap();
        let sensor_x = &cap[1].parse::<i32>().unwrap();
        let sensor_y = &cap[2].parse::<i32>().unwrap();
        let beacon_x = &cap[3].parse::<i32>().unwrap();
        let beacon_y = &cap[4].parse::<i32>().unwrap();
        let sensor = Sensor { x: *sensor_x, y: *sensor_y };
        let beacon = Beacon { x: *beacon_x, y: *beacon_y };
        readings.insert(sensor, beacon);
        sensor.distance(&beacon);
    }

    if part == 1 {

        let mut marked_points = HashSet::new();
        let mut i = 0;
        for (sensor, beacon) in &readings {
            dbg!(i);
            let d = sensor.distance(&beacon);
            let impossible_positions = sensor
                .points_within_in_line(d, considered_line)
                .into_iter()
                .filter(|p| p.y == considered_line )
                .filter(|p| p != beacon );
            for p in impossible_positions {
                marked_points.insert(p);
            }
            i += 1;
        }
        println!("There are {0} points on line {considered_line}", marked_points.len());
    } else {
        let rangey = 0..=400000;
        let rangex = 0..=4000000;
        for y in rangey {
            if y % 4000 == 0 {
              println!("Considering line {y}");
            }
            for p in free_positions(y, &rangex, &readings) {
                println!("{p:?} is free");
            }
        }
    }
}

fn free_positions(y: i32, x_range: &std::ops::RangeInclusive<i32>, readings: &HashMap<Point,Point>) -> Vec<Point> {
    let mut free_pos = vec![];
    let mut x = *x_range.start();
    while x <= *x_range.end() {
        let current = Point { x, y };

        // FIXME: maybe we could sort sensors from right to left to maximize our jump size 🤯
        let within_reach = readings.iter().filter(|(sensor, beacon)| current.distance(sensor) <= sensor.distance(beacon));
        let mut w = vec![];
        for (sensor, _) in within_reach {
            w.push(sensor);
        }
        w.sort_by(|s1,s2| s2.x.cmp(&s1.x));
        match w.get(0) {
            None => free_pos.push(current),
            Some(sensor) => {
                // then we know current point cannot be the missing beacon
                // we also know we can progress on the line quite a while
                if x < sensor.x {
                    // println!("Moving from pos {x} to {0} because in the range of {1:?}", x + 2 * (sensor.x - x), sensor);
                    x += 2 * (sensor.x - x);
                } else {
                    // println!("{0} sensors were in reach of this point ({current:?}). Example: {1:?}", w.len(), w.get(0).unwrap());
                    let beacon = readings.get(*sensor).unwrap();
                    x += (sensor.distance(beacon) - current.distance(sensor)) as i32;
                }
            }
        }
        x += 1;
    }
    free_pos
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

type Sensor = Point;
type Beacon = Point;

impl Point {
    fn distance(&self, other: &Point) -> u32 {
        let d = (self.x - other.x).abs() + (self.y - other.y).abs();
        assert!(d >= 0);
        d as u32
    }

    fn points_within_in_line(&self, udistance: u32, y: i32) -> Vec<Point> {
        let mut points = vec![];
        let distance = udistance as i32;
        for x in (self.x-distance)..=(self.x+distance) {
            let miny = self.y - distance + (x - self.x).abs();
            let maxy = self.y + distance - (x - self.x).abs();
            if (miny..=maxy).contains(&y) {
                let p = Point { x, y };
                assert!(self.distance(&p) <= udistance); // just in case I made a one-off error
                points.push(p);
            }
        }
        points
    }

    fn points_within(&self, udistance: u32) -> Vec<Point> {
        let mut points = vec![];
        let distance = udistance as i32;
        for x in (self.x-distance)..=(self.x+distance) {
            let miny = self.y - distance + (x - self.x).abs();
            let maxy = self.y + distance - (x - self.x).abs();
            for y in miny..=maxy {
                let p = Point { x, y };
                assert!(self.distance(&p) <= udistance); // just in case I made a one-off error
                points.push(p);
            }
        }
        points
    }
}
