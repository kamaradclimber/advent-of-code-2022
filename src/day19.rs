use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut blueprints = vec![];
    let re = Regex::new(r"Each [^\.]+ costs [^\.]+\.").unwrap();
    for (idx, line) in lines.enumerate() {
        println!("Line is {0}", &line);
        let mut blueprint = BluePrint { id: idx + 1, recipes: vec![] };
        for cap in re.captures_iter(line) {
            let recipe = cap[0].parse().unwrap();
            blueprint.recipes.push(recipe);
            // optim 3: consider building geode robots first: no significant improvement
            // blueprint.recipes.sort_by(|r1, r2| r2.importance().cmp(&r1.importance()));
        }
        dbg!(&blueprint);
        blueprints.push(blueprint);
    }

    // toy a bit with blueprint 1
    let b = &blueprints[0];

    let mut quality_level = 0;

    let mut tested_count = 0;
    for b in blueprints {
        let max_consumable_ore_per_minute = b.recipes.iter().map(|recipe| recipe.cost_in(&Resource::Ore)).max().expect("Every blueprint has a recipe consuming ore");

        let mut already_tested = HashSet::new();
        let mut possible_futures = vec![]; // let's use a stack to explore with DFS to get a lower list
        possible_futures.push(Stock::start());
        let mut geode_max = 0;
        loop {
            // println!("Future to explore: {0}", possible_futures.len());
            match possible_futures.pop() {
                None => break, // we are finished exploring all futures
                Some(possible_future) => {
                    //println!("We consider a future at time: {0}", possible_future.minute);
                    // optim 1: no test to redo the same amount of work
                    if already_tested.contains(&possible_future) {
                        continue;
                    } else {
                        already_tested.insert(possible_future);
                    }
                    // optim 2: no need to get further: from 10s to 1s
                    if possible_future.potential_geode_built() < geode_max {
                        // println!("Cutting this branch because there is no time to build enough geodes");
                        continue;
                    }
                    if possible_future.minute >= 24 {
                        if possible_future.geode > geode_max {
                            geode_max = possible_future.geode;
                            // println!("Found a path with a better geocode count: {0}", geode_max);
                            // dbg!(possible_future);
                        }
                    } else {
                        tested_count += 1;
                        for o in options(&possible_future, &b) {
                            // optim 4: dump resources we can't use. It will helping with caching.
                            // Doing this for ore only, divides by 4 time to run the demo and by 9
                            // the real input
                            let o = o.trim(max_consumable_ore_per_minute * (24 - o.minute));
                            possible_futures.push(o);
                        }
                    }
                }
            }
        }
        println!("Best geobuilt for {0}: {geode_max}", b.id);
        quality_level += (b.id as u32) * geode_max;
    }
    println!("DEBUG: We tested {tested_count} situation total");
    println!("Total quality level is {quality_level}");
}

struct BluePrint {
    id: usize, // just for final computation
    recipes: Vec<Recipe>,
}

impl std::fmt::Debug for BluePrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Blueprint {0}:", self.id)?;
        for recipe in &self.recipes {
            write!(f, "{:?}", recipe)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl std::str::FromStr for Robot {
    type Err = &'static str;
    fn from_str(robot: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        match robot {
            "geode" => Ok(Robot::Geode),
            "clay" => Ok(Robot::Clay),
            "ore" => Ok(Robot::Ore),
            "obsidian" => Ok(Robot::Obsidian),
            _ => Err("unknown robot type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
}
impl std::str::FromStr for Resource {
    type Err = &'static str;
    fn from_str(resource_str: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        match resource_str {
            "ore" => Ok(Resource::Ore),
            "clay" => Ok(Resource::Clay),
            "obsidian" => Ok(Resource::Obsidian),
            _ => Err("unknown resource type"),
        }
    }
}

// let's have a way to represent our stock that is easy to copy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stock {
    minute: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl Stock {
    fn start() -> Stock {
        Stock {
            minute: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
    fn amount_of(&self, resource: Resource) -> u32 {
        match resource {
            Resource::Ore => self.ore,
            Resource::Clay => self.clay,
            Resource::Obsidian => self.obsidian,
        }
    }

    fn consume(self, resource: Resource, amount: u32) -> Stock {
        match resource {
            Resource::Ore => Stock { ore: self.ore - amount, ..self },
            Resource::Clay => Stock { clay: self.clay - amount, ..self },
            Resource::Obsidian => Stock {
                obsidian: self.obsidian - amount,
                ..self
            },
        }
    }

    fn gain(self, robot: Robot) -> Stock {
        match robot {
            Robot::Ore => Stock {
                ore_robots: self.ore_robots + 1,
                ..self
            },
            Robot::Clay => Stock {
                clay_robots: self.clay_robots + 1,
                ..self
            },
            Robot::Obsidian => Stock {
                obsidian_robots: self.obsidian_robots + 1,
                ..self
            },
            Robot::Geode => Stock {
                geode_robots: self.geode_robots + 1,
                ..self
            },
        }
    }

    fn apply(&self, recipe: &Recipe) -> Stock {
        // TODO: this is a good candidate to use fold/reduce/inject equivalent
        let mut s = *self;
        for &(r, a) in &recipe.resources {
            s = s.consume(r, a);
        }
        s.gain(recipe.produce)
    }

    fn tick(self) -> Stock {
        Stock {
            minute: self.minute + 1,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            ..self
        }
    }

    fn potential_geode_built(&self) -> u32 {
        let remaining_time = 24 - self.minute;
        let mut pot = self.geode;
        // best we can do is to build one robot per turn until the end
        for i in 0..remaining_time {
            pot += self.geode_robots + i;
        }
        pot
    }

    fn trim(&self, max_consumable_ore: u32) -> Stock {
        // restrict to resources that can be consumed in the time
        Stock {
            ore: std::cmp::min(self.ore, max_consumable_ore),
            ..*self
        }
    }
}

fn options(stock: &Stock, blueprint: &BluePrint) -> Vec<Stock> {
    let mut opts = vec![];
    for recipe in &blueprint.recipes {
        if recipe.available(stock) {
            opts.push(stock.tick().apply(&recipe));
        }
    }
    opts.push(stock.tick()); // we could also do nothing
    opts
}

#[derive(Clone)]
struct Recipe {
    resources: Vec<(Resource, u32)>,
    produce: Robot,
}

impl Recipe {
    fn empty() -> Recipe {
        Recipe {
            resources: vec![],
            produce: Robot::Ore,
        }
    }

    fn cost_in(&self, resource: &Resource) -> u32 {
        self.resources.iter().filter(|&(r, _)| r == resource).map(|&(_, amount)| amount).next().unwrap_or(0)
    }

    fn importance(&self) -> u32 {
        // the more important means we would like to do this recipe first
        match self.produce {
            Robot::Geode => 5,
            _ => 1,
        }
    }

    fn available(&self, stock: &Stock) -> bool {
        self.resources.iter().all(|&(resource, amount)| stock.amount_of(resource) >= amount)
    }
}

impl std::str::FromStr for Recipe {
    type Err = &'static str;
    fn from_str(recipe_str: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let mut recipe = Recipe::empty();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }
        for cap in RE.captures_iter(recipe_str) {
            let amount = cap[1].parse::<u32>().unwrap();
            let recipient = cap[2].parse()?;
            recipe.resources.push((recipient, amount));
        }
        let re = Regex::new(r"Each (.+) robot").unwrap();
        let cap = re.captures(recipe_str).expect("Each recipe must produces a robot");
        let robot = cap[1].parse()?;
        recipe = Recipe { produce: robot, ..recipe };
        Ok(recipe)
    }
}
impl std::fmt::Debug for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, " Each {0:?} robot costs ", self.produce)?;
        for (i, (recipient, amount)) in self.resources.iter().enumerate() {
            write!(f, "{0} {1:?}", amount, recipient)?;
            if i < self.resources.len() - 1 {
                write!(f, " and ")?;
            }
        }
        write!(f, ".")?;
        Ok(())
    }
}
