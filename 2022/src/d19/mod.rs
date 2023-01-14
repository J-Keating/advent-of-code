use std::{fs, collections::{VecDeque, HashMap}, time::Instant};
use ::function_name::named;
use itertools::Itertools;
use regex::Regex;
#[allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Debug)]
enum MineralType {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
    COUNT = 4,
}

trait MineralStateOperations {
    fn new() -> MineralState;
    fn get(&self, mineral_type: MineralType) -> i32;
    fn set(&mut self, mineral_type: MineralType, value: i32);
    fn add(&mut self, mineral_type: MineralType, value: i32);
    fn add_all(&mut self, other: &MineralState);
    fn sub_all(&mut self, other: &MineralState);
    fn can_afford(&self, other: &MineralState) -> bool;
    fn all_less_or_equal(&self, other: &MineralState) -> bool;
}

type MineralState = [i32; MineralType::COUNT as usize];
impl MineralStateOperations for MineralState {
    fn new() -> MineralState {
        [0, 0, 0, 0]
    }

    fn get(&self, mineral_type: MineralType) -> i32 {
        self[mineral_type as usize]
    }

    fn set(&mut self, mineral_type: MineralType, value: i32) {
        self[mineral_type as usize] = value;
    }

    fn add(&mut self, mineral_type: MineralType, value: i32) {
        self[mineral_type as usize] += value;
    }

    fn add_all(&mut self, other: &MineralState) {
        for i in 0..MineralType::COUNT as usize {
            self[i] += other[i];
        }
    }

    fn sub_all(&mut self, other: &MineralState) {
        for i in 0..MineralType::COUNT as usize {
            self[i] -= other[i];
        }
    }

    fn can_afford(&self, other: &MineralState) -> bool {
        for i in 0..MineralType::COUNT as usize {
            if self[i] < other[i] {
                return false;
            }
        }
        true
    }

    fn all_less_or_equal(&self, other: &MineralState) -> bool {
        for i in 0..MineralType::COUNT as usize {
            if self[i] > other[i] {
                return false;
            }
        }
        true
    }
}
type MineralCost = MineralState;
type Blueprint = [MineralCost; MineralType::COUNT as usize];

#[derive(Copy, Clone, Debug)]
struct WorldState {
    pub minute: i32,
    pub robot_count: MineralState,
    pub ore_count: MineralState,
}

impl WorldState {
    fn new() -> WorldState {
        let mut ret = WorldState {
            minute: 0,
            robot_count: MineralState::new(),
            ore_count: MineralState::new(),
        };
        ret.robot_count[MineralType::Ore as usize] = 1;
        ret
    }

    fn tick_minute(&mut self) -> &mut Self {
        self.minute += 1;
        self.ore_count.add_all(&self.robot_count);
        self
    }

    fn spend_minerals(&mut self, cost: &MineralCost) -> &mut Self {
        self.ore_count.sub_all(cost);
        self
    }

    fn add_robot(&mut self, mineral_type: MineralType) -> &mut Self {
        self.robot_count.add(mineral_type, 1);
        self
    }

    fn check(&self, minute: i32, robot_count: MineralState, ore_count: MineralState) -> bool {
        self.minute == minute && self.robot_count == robot_count && self.ore_count == ore_count
    }
}

fn print_worldstates(worldstates: &VecDeque<WorldState>) {
    for ws in worldstates {
        println!("Minute: {}, Robots: {:?}, Ore: {:?}", ws.minute, ws.robot_count, ws.ore_count);
    }
}

#[test]
fn repro_sample() {
    let costs = blueprint_from_line("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
    let mut ws: WorldState = WorldState::new();
    ws.tick_minute();
    assert!(ws.check(1, [1, 0, 0, 0], [1, 0, 0, 0]));
    ws.tick_minute();
    assert!(ws.check(2, [1, 0, 0, 0], [2, 0, 0, 0]));
    ws.spend_minerals(&costs[MineralType::Clay as usize]).tick_minute().add_robot(MineralType::Clay);
    assert!(ws.check(3, [1, 1, 0, 0], [1, 0, 0, 0]));
    ws.tick_minute();
    assert!(ws.check(4, [1, 1, 0, 0], [2, 1, 0, 0]));
    ws.spend_minerals(&costs[MineralType::Clay as usize]).tick_minute().add_robot(MineralType::Clay);
    assert!(ws.check(5, [1, 2, 0, 0], [1, 2, 0, 0]));
    ws.tick_minute();
    assert!(ws.check(6, [1, 2, 0, 0], [2, 4, 0, 0]));
    ws.spend_minerals(&costs[MineralType::Clay as usize]).tick_minute().add_robot(MineralType::Clay);
    assert!(ws.check(7, [1, 3, 0, 0], [1, 6, 0, 0]));
    ws.tick_minute();
    assert!(ws.check(8, [1, 3, 0, 0], [2, 9, 0, 0]));
    ws.tick_minute();
    assert!(ws.check(9, [1, 3, 0, 0], [3, 12, 0, 0]));
    ws.tick_minute();
    assert!(ws.check(10, [1, 3, 0, 0], [4, 15, 0, 0]));
    ws.spend_minerals(&costs[MineralType::Obsidian as usize]).tick_minute().add_robot(MineralType::Obsidian);
    assert!(ws.check(11, [1, 3, 1, 0], [2, 4, 0, 0]));
    ws.spend_minerals(&costs[MineralType::Clay as usize]).tick_minute().add_robot(MineralType::Clay);
    assert!(ws.check(12, [1, 4, 1, 0], [1, 7, 1, 0]));
    ws.tick_minute();
    assert!(ws.check(13, [1, 4, 1, 0], [2, 11, 2, 0]));
    ws.tick_minute();
    assert!(ws.check(14, [1, 4, 1, 0], [3, 15, 3, 0]));
    ws.spend_minerals(&costs[MineralType::Obsidian as usize]).tick_minute().add_robot(MineralType::Obsidian);
    assert!(ws.check(15, [1, 4, 2, 0], [1, 5, 4, 0]));
    ws.tick_minute();
    assert!(ws.check(16, [1, 4, 2, 0], [2, 9, 6, 0]));
    ws.tick_minute();
    assert!(ws.check(17, [1, 4, 2, 0], [3, 13, 8, 0]));
    ws.spend_minerals(&costs[MineralType::Geode as usize]).tick_minute().add_robot(MineralType::Geode);
    assert!(ws.check(18, [1, 4, 2, 1], [2, 17, 3, 0]));
    ws.tick_minute();
    assert!(ws.check(19, [1, 4, 2, 1], [3, 21, 5, 1]));
    ws.tick_minute();
    assert!(ws.check(20, [1, 4, 2, 1], [4, 25, 7, 2]));
}

#[test]
fn mineralstate_compare() {
    let x: MineralState = [1, 2, 3, 4];
    assert!(x.can_afford( &[1, 2, 3, 4] ));
    assert!(!x.can_afford( &[1, 2, 3, 5] ));
    assert!(x.can_afford( &[0, 2, 3, 4] ));
}

fn blueprint_from_line(line: &str) -> Blueprint {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let x = re.captures(line).unwrap();
    let mut new_blueprint : Blueprint = [MineralCost::new(); MineralType::COUNT as usize];
    // Each ore robot costs 4 ore.
    new_blueprint[MineralType::Ore as usize][MineralType::Ore as usize] = x[2].parse::<i32>().unwrap();
    // Each clay robot costs 2 ore.
    new_blueprint[MineralType::Clay as usize][MineralType::Ore as usize] = x[3].parse::<i32>().unwrap();
    // Each obsidian robot costs 3 ore and 14 clay.
    new_blueprint[MineralType::Obsidian as usize][MineralType::Ore as usize] = x[4].parse::<i32>().unwrap();
    new_blueprint[MineralType::Obsidian as usize][MineralType::Clay as usize] = x[5].parse::<i32>().unwrap();
    // Each geode robot costs 2 ore and 7 obsidian.
    new_blueprint[MineralType::Geode as usize][MineralType::Ore as usize] = x[6].parse::<i32>().unwrap();
    new_blueprint[MineralType::Geode as usize][MineralType::Obsidian as usize] = x[7].parse::<i32>().unwrap();
    new_blueprint
}

fn load_data(path: &str) -> Vec<Blueprint> {
    let mut ret: Vec<Blueprint> = Vec::new();
    for line in fs::read_to_string(path).expect("Error loading file").split("\r\n") {
        ret.push(blueprint_from_line(line));
    }
    ret
}

fn cull_bad_worldstate_options(in_options: VecDeque<WorldState>, total_minutes: i32) -> VecDeque<WorldState> {
    let mut out_options: VecDeque<WorldState> = VecDeque::new();
    let minute = in_options[0].minute;
    // Want to cull bad options...
    // For all states with identical robot counts, we can drop ones with clearly worse ore counts
    let mut robot_count_map: HashMap<MineralCost, Vec<MineralCost>> = HashMap::new();
    for state in in_options {
        let robot_count = state.robot_count;
        let ore_count = state.ore_count;
        if robot_count_map.contains_key(&robot_count) {
            let ore_counts = robot_count_map.get_mut(&robot_count).unwrap();
            // If any one of the ore counts is better (greater or equal for all ores) than our new one
            // then we're done with it
            if ore_counts.iter().filter(|c| c.can_afford(&ore_count)).count() > 0 {
                continue
            }
            // If our new one is better (greater or equal for all ores) than any of the current ones,
            // the we're going to add it.  But first, remove any smaller than it.
            let mut new_counts = ore_counts.iter().filter(|&c| !ore_count.can_afford(c)).map(|c| *c).collect_vec();// = foo_iter.collect_vec(); //foo_iter.filter(|&&c| !ore_count.can_afford(&c)).collect_vec();
            new_counts.push(ore_count);
            *ore_counts = new_counts;
        } else {
            robot_count_map.insert(robot_count, vec![ore_count]);
        }
    }
    for (robot_count, ore_counts) in &robot_count_map {
        for ore_count in ore_counts {
            out_options.push_back(WorldState { minute, robot_count: *robot_count, ore_count: *ore_count });
        }
    }

    // Geodes are the goal.  Eliminate scenarios where we can't get enough geodes in the remaining time.
    let minutes_left = total_minutes - minute;
    // If a robot was made every remaining minute, how many geodes would that be?
    // This is the minimum goeodes that could be added to any existing option in the remaining time.
    let max_possible_additional_geodes_from_new_robots = (minutes_left * (minutes_left+1)) / 2;
    // If each of the the existing options made no more geode robots at all, how many geodes would they end up with?
    // This is the minimum number of geodes that we will get from of the existing options.
    let min_final_geodes_from_current_options = out_options.iter().map(|x| x.ore_count[MineralType::Geode as usize] + x.robot_count[MineralType::Geode as usize] * minutes_left).max().unwrap();
    let before_count = out_options.len();
    out_options = out_options.into_iter().filter(|x| {
        // If we added the *max* number of geodes possible in the remaining time and it's *still*
        // less than the minimum number of geodes we can get from the current options, then we can
        // eliminate the option.
        let max_from_this = x.ore_count[MineralType::Geode as usize] + (x.robot_count[MineralType::Geode as usize] * minutes_left) + max_possible_additional_geodes_from_new_robots;
        max_from_this >= min_final_geodes_from_current_options
    }).collect();
    if before_count != out_options.len() {
        //println!("Culled {} options to {} options", before_count, out_options.len());
    }

    out_options
}

fn most_geodes_from_blueprint(robot_costs: &Blueprint, total_minutes: i32) -> i32 {
    let mut options: VecDeque<WorldState> = VecDeque::new();
    options.push_back(WorldState::new());
    let mut last_minute = options.get(0).unwrap().minute;
    while options.get(0).unwrap().minute < total_minutes {
        let minute = options.get(0).unwrap().minute;
        if minute != last_minute {
            //println!("Minute {}: {}", minute, options.len());
            last_minute = minute;
            assert!(options.len() == options.iter().filter(|x| x.minute == last_minute).count());
            // println!("Culling {} options:", options.len());
            // println!("  Before");
            // print_worldstates(&options);
            options = cull_bad_worldstate_options(options, total_minutes);
            // println!("  After");
            // print_worldstates(&options);
        }
        let state = options.pop_front().unwrap();
        if state.ore_count.can_afford(&robot_costs[MineralType::Geode as usize]) {
            // If we can afford a geode-cracking robot, it's always the right thing to do
            let new_state = *state.clone().spend_minerals(&robot_costs[MineralType::Geode as usize]).tick_minute().add_robot(MineralType::Geode);
            options.push_back(new_state);
        }
        else {
            // If we can't afford a geode robot, the we need to consider other things...
            let mut cant_afford_count = 0;
            for i in [MineralType::Obsidian, MineralType::Clay, MineralType::Ore] {
                if state.ore_count.can_afford(&robot_costs[i as usize]) {
                    let new_state = *state.clone().spend_minerals(&robot_costs[i as usize]).tick_minute().add_robot(i);
                    options.push_back(new_state);
                }
                else {
                    cant_afford_count += 1;
                }
            }
            // If we can afford all of the robots, then we should have bought one of them, so doing nothing is a bad idea
            if cant_afford_count != 0 {
                options.push_back(*state.clone().tick_minute());
            }
        }
    }
    assert!(options.len() == options.iter().filter(|x| x.minute == total_minutes).count());
    options.iter().map(|x| x.ore_count[MineralType::Geode as usize]).max().unwrap()
}

#[named]
fn part1() {
    let now = Instant::now();
    let blueprints = load_data("src\\d19\\data.txt");
    let mut total_quality_level = 0;
    for (i, b) in blueprints.iter().enumerate() {
        print!("{}: {:?} => ", i, b);
        let best_geode_count = most_geodes_from_blueprint(&b, 24);
        println!("{}", best_geode_count);
        total_quality_level += best_geode_count * (i as i32 + 1);
    }

    println!("{}: {} ({} sec)", function_name!(), total_quality_level, now.elapsed().as_secs_f32());
}
    
#[named]
fn part2() {
    let now = Instant::now();
    let blueprints = load_data("src\\d19\\data.txt");

    let mut total_quality_level = 1;
    for (i, b) in blueprints[0..3].iter().enumerate() {
        print!("{}: {:?} => ",i, b);
        let best_geode_count = most_geodes_from_blueprint(&b, 32);
        println!("{}", best_geode_count);
        total_quality_level *= best_geode_count;
    }

    println!("{}: {} ({} sec)", function_name!(), total_quality_level, now.elapsed().as_secs_f32());
}

pub fn run() {
    part1();
    part2();
}

// part1: 1624 (60.739445 sec)
// part2: 12628 (84.307846 sec)