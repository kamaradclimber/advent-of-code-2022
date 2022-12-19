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
        let mut blueprint = BluePrint { id: idx + 1, recipes: vec![] };
        for cap in re.captures_iter(line) {
            let recipe = cap[0].parse().unwrap();
            blueprint.recipes.push(recipe);
        }
        blueprints.push(blueprint);
    }

    let max_time = if part == 1 { 24 } else { 32 };
    let take = if part == 1 { 1000 } else { 3 };

    let mut quality_level = 0;
    let mut product = 1;

    let mut tested_count = 0;

    for b in blueprints.iter().take(take) {
        let max_consumable_ore_per_minute = b.recipes.iter().map(|recipe| recipe.cost_in(&Resource::Ore)).max().expect("at least one recipe per blueprint");
        let max_consumable_clay_per_minute = b.recipes.iter().map(|recipe| recipe.cost_in(&Resource::Clay)).max().expect("at least one recipe per blueprint");
        let max_consumable_obsidian_per_minute = b.recipes.iter().map(|recipe| recipe.cost_in(&Resource::Obsidian)).max().expect("at least one recipe per blueprint");

        let mut already_tested = HashSet::new();
        let mut possible_futures = vec![]; // let's use a stack to explore with DFS to get a lower list
        possible_futures.push(Stock::start(max_time));
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
                    if possible_future.minute >= max_time {
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
                            let remaining_time = max_time - o.minute;
                            let o = o.trim(
                                max_consumable_ore_per_minute * remaining_time,
                                max_consumable_clay_per_minute * remaining_time,
                                max_consumable_obsidian_per_minute * remaining_time,
                            );
                            // optim 5: avoid constructing more robots if we are already producing
                            // enough to use in the most heavy recipe. This moves part 2 on real
                            // input from 19s to 7.5s
                            if possible_future.ore_robots + 1 == o.ore_robots && possible_future.ore_robots >= max_consumable_ore_per_minute {
                                continue;
                            }
                            if possible_future.clay_robots + 1 == o.clay_robots && possible_future.clay_robots >= max_consumable_clay_per_minute {
                                continue;
                            }
                            if possible_future.obsidian_robots + 1 == o.obsidian_robots && possible_future.obsidian_robots >= max_consumable_obsidian_per_minute {
                                continue;
                            }
                            possible_futures.push(o);
                        }
                    }
                }
            }
        }
        // println!("Best geobuilt for {0}: {geode_max}", b.id);
        quality_level += (b.id as u32) * geode_max;
        product = product * geode_max;
    }
    println!("DEBUG: We tested {tested_count} situation total");
    if part == 1 {
        println!("Total quality level is {quality_level}");
    } else {
        println!("Product is {product}");
    }
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
    max_time: u32,
}

impl Stock {
    fn start(max_time: u32) -> Stock {
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
            max_time: max_time,
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
        let remaining_time = self.max_time - self.minute;
        let mut pot = self.geode;
        // best we can do is to build one robot per turn until the end
        for i in 0..remaining_time {
            pot += self.geode_robots + i;
        }
        pot
    }

    fn trim(&self, max_consumable_ore: u32, max_consumable_clay: u32, max_consumable_obsidian: u32) -> Stock {
        // restrict to resources that can be consumed in the time
        Stock {
            ore: std::cmp::min(self.ore, max_consumable_ore),
            clay: std::cmp::min(self.clay, max_consumable_clay),
            obsidian: std::cmp::min(self.obsidian, max_consumable_obsidian),
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
