use std::{fs, collections::VecDeque, time::Instant};
use ::function_name::named;
use regex::Regex;

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
}
type MineralCost = MineralState;
type Blueprint = [MineralCost; MineralType::COUNT as usize];

#[derive(Copy, Clone, Debug)]
struct WorldState {
    pub minute: i32,
    pub robot_count: MineralState,
    pub ore_count: MineralState,
    pub last_robot_added: Option<MineralType>,
}

impl WorldState {
    fn new() -> WorldState {
        let mut ret = WorldState {
            minute: 0,
            robot_count: MineralState::new(),
            ore_count: MineralState::new(),
            last_robot_added: None,
        };
        ret.robot_count[MineralType::Ore as usize] = 1;
        ret
    }

    fn tick_minute(&mut self) -> &mut Self {
        self.minute += 1;
        self.ore_count.add_all(&self.robot_count);
        self.last_robot_added = None;
        self
    }

    fn spend_minerals(&mut self, cost: &MineralCost) -> &mut Self {
        self.ore_count.sub_all(cost);
        self
    }

    fn add_robot(&mut self, mineral_type: MineralType) -> &mut Self {
        self.robot_count.add(mineral_type, 1);
        self.last_robot_added = Some(mineral_type);
        // if self.robot_count[MineralType::Geode as usize] > 1 {
        //     panic!("We have more than one geode robot!");
        // }
        self
    }

    fn state_last_minute(&self) -> WorldState {
        assert!(self.last_robot_added == None);
        let mut ret = *self;
        ret.minute -= 1;
        ret.ore_count.sub_all(&self.robot_count);
        ret
    }

    fn check(&self, minute: i32, robot_count: MineralState, ore_count: MineralState) -> bool {
        self.minute == minute && self.robot_count == robot_count && self.ore_count == ore_count
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

const MINUTES: i32 = 24;
fn most_geodes_from_blueprint(robot_costs: &Blueprint) -> i32 {
    let mut options: VecDeque<WorldState> = VecDeque::new();
    options.push_back(WorldState::new());
    let mut last_minute = options.get(0).unwrap().minute;
    while options.get(0).unwrap().minute < MINUTES {
        let state = options.pop_front().unwrap();
        if state.minute != last_minute {
            println!("Minute {}: {}", state.minute, options.len());
            last_minute = state.minute;
        }
        if state.ore_count.can_afford(&robot_costs[MineralType::Geode as usize]) {
            // If we can afford a geode-cracking robot, it's always the right thing to do
            let new_state = *state.clone().spend_minerals(&robot_costs[MineralType::Geode as usize]).tick_minute().add_robot(MineralType::Geode);
            // if last_minute == 17 {
            //     println!("{}: {:?}", new_state.minute, new_state);
            // }
            options.push_back(new_state);
        }
        else {
            // If we can't afford a geode robot, the we need to consider other things...
            let mut cant_afford_count = 0;
            for i in [MineralType::Obsidian, MineralType::Clay, MineralType::Ore] {
                if state.ore_count.can_afford(&robot_costs[i as usize]) {
                    if state.last_robot_added == None && state.state_last_minute().ore_count.can_afford(&robot_costs[i as usize]) {
                        continue;
                    }
                    let new_state = *state.clone().spend_minerals(&robot_costs[i as usize]).tick_minute().add_robot(i);
                    //let existing = options.iter().find(|x| x.minute == new_state.minute && x.robot_count == new_state.robot_count);
                    options.push_back(new_state);
                    // if existing.is_none() {
                    // }
                }
                else {
                    cant_afford_count += 1;
                }
            }
            // If we can afford all of the robots, then we should have bought one of them, so doing nothing is a bad idea
            if cant_afford_count != 0 {
                options.push_back(*state.clone().tick_minute());
                // let new_state = *state.clone().tick_minute();
                // let existing = options.iter().find(|x| x.minute == new_state.minute && x.robot_count == new_state.robot_count);
                // if existing.is_none() {
                //     options.push_back(new_state);
                // }
            }
        }
    }
    assert!(options.len() == options.iter().filter(|x| x.minute == MINUTES).count());
    options.iter().map(|x| x.ore_count[MineralType::Geode as usize]).max().unwrap()
}

#[named]
fn part1() {
    let now = Instant::now();
    let blueprints = load_data("src\\d19\\data.txt");
    let mut total_quality_level = 0;
    for (i, b) in blueprints.iter().enumerate() {
        println!("{:?} => ", b);
        let best_geode_count = most_geodes_from_blueprint(&b);
        println!("{}", best_geode_count);
        total_quality_level += best_geode_count * (i as i32 + 1);
    }

    println!("{}: {} ({} sec)", function_name!(), total_quality_level, now.elapsed().as_secs_f32());
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