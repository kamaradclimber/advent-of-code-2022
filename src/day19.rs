use lazy_static::lazy_static;
use regex::Regex;
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
        }
        dbg!(&blueprint);
        blueprints.push(blueprint);
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
struct Stock {
    resources: Vec<(Resource, u32)>,
    robots: Vec<(Robot, u32)>,
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
