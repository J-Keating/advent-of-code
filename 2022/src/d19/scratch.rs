use std::{fs, io::BufRead, cmp, collections::VecDeque};
use ::function_name::named;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct OreCost {
    pub ore_cost: i32,
}

#[derive(Copy, Clone, Debug)]
struct ObsidianRobotCost {
    pub ore_cost: i32,
    pub clay_cost: i32
}

#[derive(Copy, Clone, Debug)]
struct GeodeRobotCost {
    pub ore_cost: i32,
    pub obsidian_cost: i32
}

#[derive(Copy, Clone, Debug)]
struct Blueprint {
    pub ore_robot_cost: OreCost,
    pub clay_robot_cost: OreCost,
    pub obsidian_robot_cost: ObsidianRobotCost,
    pub geode_robot_cost: GeodeRobotCost,
}

#[derive(Copy, Clone, Debug)]
struct WorldState {
    minute: i32,
    ore_robot_count: i32,
    clay_robot_count: i32,
    obsidian_robot_count: i32,
    geode_robot_count: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl WorldState {
    fn new() -> WorldState {
        WorldState {
            minute: 0,
            ore_robot_count: 0,
            clay_robot_count: 0,
            obsidian_robot_count: 0,
            geode_robot_count: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn tick_minute(&mut self) {
        self.minute += 1;
        self.ore += self.ore_robot_count;
        self.clay += self.clay_robot_count;
        self.obsidian += self.obsidian_robot_count;
        self.geode += self.geode_robot_count;
    }
}

fn load_data(path: &str) -> Vec<Blueprint> {
    let mut ret: Vec<Blueprint> = Vec::new();
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    for line in fs::read_to_string(path).expect("Error loading file").split("\r\n") {
        let x = re.captures(line).unwrap();
        let new_blueprint = Blueprint {
            ore_robot_cost: OreCost { ore_cost: x[2].parse::<i32>().unwrap() },
            clay_robot_cost: OreCost { ore_cost: x[3].parse::<i32>().unwrap() },
            obsidian_robot_cost: ObsidianRobotCost { ore_cost: x[4].parse::<i32>().unwrap(), clay_cost: x[5].parse::<i32>().unwrap() },
            geode_robot_cost: GeodeRobotCost { ore_cost: x[6].parse::<i32>().unwrap(), obsidian_cost: x[7].parse::<i32>().unwrap() }};
        ret.push(new_blueprint);
    }
    ret
}

#[named]
fn part1() {
    let blueprints = load_data("src\\d19\\data.txt");
    let mut visible_face_count = 0;
    for b in blueprints {
        println!("{:?}", b);
    }

    println!("{}: {}", function_name!(), visible_face_count);
}

#[named]
fn part2() {
    let blueprints = load_data("src\\d19\\data_test.txt");
    let mut exterior_face_count = 0;

    println!("{}: {}", function_name!(), exterior_face_count);
}

pub fn run() {
    part1();
    part2();
}

// part1: 4604
// part2: 2604