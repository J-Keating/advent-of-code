use std::{fs, collections::HashMap, hash::Hash, cmp};

use ::function_name::named;
use itertools::Itertools;

//use itertools::Itertools;
//use lazy_static::lazy_static;
use utilities::{alloc_2d_vec, Point};

#[derive(Debug, Clone)]
struct Valve {
    rate: i32,
    open: bool,
    connections: HashMap<String, i32>,
}

fn print_valves(valves: &HashMap<String, Valve>) {
    let mut valves_vec: Vec<(&String, &Valve)> = valves.iter().collect();
    valves_vec.sort_by(|a, b| a.0.cmp(b.0));
    for (name, valve) in valves_vec.iter() {
        println!("{}: {:?}", name, valve);
    }
}

fn load_data(path: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    for line in file_contents_as_string.split("\r\n") {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let line_tok = line.split([ ' ', '=']).collect_vec();
        assert!(line_tok.len() >= 10);
        let (name, rate) = (line_tok[1], line_tok[5].replace(';', "").parse::<i32>().unwrap());
        let new_valve = valves.entry(name.to_string()).or_insert(Valve { rate, open: true, connections: HashMap::new() });
        for other in line_tok[10..].iter().map(|s| s.replace(',', "")) {
            new_valve.connections.insert(other, 1);
        }
    }
    valves
}

fn remove_valve(valves: &mut HashMap<String, Valve>, name_to_remove: &str) {
    let valve_to_remove = valves.remove(name_to_remove).unwrap();
    for (name, valve) in valves.iter_mut() {
        match valve.connections.remove(name_to_remove) {
            Some(cost) => {
                for (other_name, other_cost) in valve_to_remove.connections.iter() {
                    let new_cost = cost + other_cost;
                    match valve.connections.get_mut(other_name) {
                        Some(old_cost) => {
                            if new_cost < *old_cost {
                                *old_cost = new_cost;
                            }
                        },
                        None => {
                            if other_name != name {
                                valve.connections.insert(other_name.to_string(), new_cost);
                            }
                        },
                    }
                }
            },
            None => {},
        }
    }
}

fn reduce_graph(valves: &mut HashMap<String, Valve>) {
    let nodes_to_remove = valves.iter().filter(|(name, valve)| *name != "AA" && valve.rate == 0).map(|(name, _)| name.to_string()).collect_vec();
    for name in nodes_to_remove {
        remove_valve(valves, &name);
        // println!("{} removed:", name);
        // print_valves(&valves);
    }
}

fn best_result_from(valves: &mut HashMap<String, Valve>, name_current: &str, remaining_time: i32) -> i32 {
    let valve_copy = valves.get_mut(name_current).unwrap().clone();
    if remaining_time <= 1 || valve_copy.open {
        0
    } else if remaining_time == 2 {
        valve_copy.rate * (remaining_time - 1)
    } else {
        let mut best = (0, false);
        // How do we do if we don't open this valve?
        for (name_next, cost) in valve_copy.connections.iter() {
            let result = best_result_from(valves, name_next, (remaining_time) - *cost);
            if result > best.0 {
                best = (result, false);
            }
        }
        // How do we do if we open this valve?
        let from_this_valve = valve_copy.rate * (remaining_time-1);
        valves.get_mut(name_current).unwrap().open = true;
        for (name_next, cost) in valve_copy.connections.iter() {
            let result = from_this_valve + best_result_from(valves, name_next, (remaining_time - 1) - *cost);
            if result > best.0 {
                best = (result, true);
            }
        }
        valves.get_mut(name_current).unwrap().open = best.1;
        println!("Best result from {} with {} time left: {} (open: {})", name_current, remaining_time, best.0, best.1);
        best.0
    }
}

#[named]
fn part1() {
    let mut valves = load_data("src\\d16\\data_test.txt");
    reduce_graph(&mut valves);
    print_valves(&valves);
    let mut candidates: HashMap<String, i32> = HashMap::new();
    let mut nodes_to_visit = vec![("AA".to_string(), 30)];
    while !nodes_to_visit.is_empty() {
        let (name, remaining_time) = nodes_to_visit.pop().unwrap();
        let valve = valves.get_mut(&name).unwrap();
        if valve.open {
            let value = valve.rate * (remaining_time-1);
            candidates.entry(name).and_modify(|c| *c = cmp::max(*c, value)).or_insert(value);
        }
        for (name, cost) in valve.connections.iter() {
            if !candidates.contains_key(name) {
                nodes_to_visit.push((name.to_string(), remaining_time - *cost));
            }
        }
    }
    println!("{}: {:?}", function_name!(), candidates);
    println!("{}: {:?}", function_name!(), valves.len());
}

#[named]
fn part2() {
    let mut valves = load_data("src\\d16\\data_test.txt");
    reduce_graph(&mut valves);
    println!("{}: {:?}", function_name!(), valves.len());
}

pub fn run() {
    part1();
    part2();
}

// part1: 779
// part2: 1


use std::{cmp, collections::HashMap, fs, time::Instant};

use ::function_name::named;
use itertools::Itertools;
use utilities::alloc_2d_vec;

//use itertools::Itertools;
//use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct Valve {
    rate: i32,
    closed: bool,
    connections: HashMap<String, i32>,
}

fn print_valves(valves: &HashMap<String, Valve>) {
    let mut valves_vec: Vec<(&String, &Valve)> = valves.iter().collect();
    valves_vec.sort_by(|a, b| a.0.cmp(b.0));
    for (name, valve) in valves_vec.iter() {
        println!("{}: {:?}", name, valve);
    }
}

fn load_data(path: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    for line in file_contents_as_string.split("\r\n") {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let line_tok = line.split([' ', '=']).collect_vec();
        assert!(line_tok.len() >= 10);
        let (name, rate) = (
            line_tok[1],
            line_tok[5].replace(';', "").parse::<i32>().unwrap(),
        );
        let new_valve = valves.entry(name.to_string()).or_insert(Valve {
            rate,
            closed: true,
            connections: HashMap::new(),
        });
        for other in line_tok[10..].iter().map(|s| s.replace(',', "")) {
            new_valve.connections.insert(other, 1);
        }
    }
    valves
}

fn remove_valve(valves: &mut HashMap<String, Valve>, name_to_remove: &str) {
    let valve_to_remove = valves.remove(name_to_remove).unwrap();
    for (name, valve) in valves.iter_mut() {
        match valve.connections.remove(name_to_remove) {
            Some(cost) => {
                for (other_name, other_cost) in valve_to_remove.connections.iter() {
                    let new_cost = cost + other_cost;
                    match valve.connections.get_mut(other_name) {
                        Some(old_cost) => {
                            if new_cost < *old_cost {
                                *old_cost = new_cost;
                            }
                        }
                        None => {
                            if other_name != name {
                                valve.connections.insert(other_name.to_string(), new_cost);
                            }
                        }
                    }
                }
            }
            None => {}
        }
    }
}

fn reduce_graph(valves: &mut HashMap<String, Valve>) {
    let nodes_to_remove = valves
        .iter()
        .filter(|(name, valve)| *name != "AA" && valve.rate == 0)
        .map(|(name, _)| name.to_string())
        .collect_vec();
    for name in nodes_to_remove {
        remove_valve(valves, &name);
    }
}

fn floyd_warshall(valves: &HashMap<String, Valve>) -> (Vec<Vec<i32>>, HashMap<String, i32>) {
    let mut distances = alloc_2d_vec(valves.len(), valves.len(), i32::max_value());
    let mut name_to_index: HashMap<String, i32> = HashMap::new();
    for (index, (name, _)) in valves.iter().enumerate() {
        name_to_index.insert(name.to_string(), index as i32);
    }
    for (name, valve) in valves.iter() {
        let index = name_to_index.get(name).unwrap();
        distances[*index as usize][*index as usize] = 0;
        for (other_name, cost) in valve.connections.iter() {
            let other_index = name_to_index.get(other_name).unwrap();
            distances[*index as usize][*other_index as usize] = *cost;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if distances[i][k] != i32::max_value() && distances[k][j] != i32::max_value() {
                    distances[i][j] = cmp::min(distances[i][j], distances[i][k] + distances[k][j]);
                }
            }
        }
    }
    (distances, name_to_index)
}

fn best_result_from_recursive_over_all_valves(
    indent_string: String,
    valves: &mut HashMap<String, Valve>,
    name_current: &str,
    name_previous: &str,
    remaining_time: i32,
) -> i32 {
    let valve_copy = valves.get_mut(name_current).unwrap().clone();
    if remaining_time <= 1 {
        return 0;
    } else if remaining_time == 2 {
        if valve_copy.closed {
            // Just enough time to open this valve and get one tick of flow
            return valve_copy.rate * (remaining_time - 1);
        }
        return 0;
    } else {
        let mut best = (0, false, "".to_string()); // (best result, leave this valve closed?, best next valve)
                                                   // How do we do if we just leave this valve closed?
        for (name_next, cost) in valve_copy.connections.iter() {
            // Don't go back to where we came from
            if name_next == name_previous {
                continue;
            }
            let result = best_result_from_recursive_over_all_valves(
                indent_string.clone() + "  ",
                valves,
                name_next,
                name_current,
                (remaining_time) - *cost,
            );
            if indent_string.len() < 4 {
                println!(
                    "{}Valve {} ({}): {} by leaving {} and going to {}",
                    indent_string,
                    name_current,
                    remaining_time,
                    result,
                    if valve_copy.closed { "closed" } else { "open " },
                    name_next
                );
            }
            if result > best.0 {
                best = (result, true, name_next.to_string()); // closed stays true
            }
        }
        // How do we do if we open this valve?
        if valve_copy.rate != 0 && valve_copy.closed {
            let from_this_valve = valve_copy.rate * (remaining_time - 1);
            valves.get_mut(name_current).unwrap().closed = false;
            for (name_next, cost) in valve_copy.connections.iter() {
                let result = from_this_valve
                    + best_result_from_recursive_over_all_valves(
                        indent_string.clone() + "  ",
                        valves,
                        name_next,
                        name_current,
                        (remaining_time - 1) - *cost,
                    );
                if indent_string.len() < 4 {
                    println!(
                        "{}Valve {} ({}): {} by {} and going to {}",
                        indent_string,
                        name_current,
                        remaining_time,
                        result,
                        "opening it",
                        name_next
                    );
                }
                if result > best.0 {
                    best = (result, false, name_next.to_string()); // closed is false
                }
            }
            valves.get_mut(name_current).unwrap().closed = true;
        }
        if indent_string.len() < 4 {
            println!(
                "{}Valve {} ({}): best: {} by {} and going to {}",
                indent_string,
                name_current,
                remaining_time,
                best.0,
                if best.1 {
                    "leaving closed"
                } else {
                    "opening it"
                },
                best.2
            );
        }
        return best.0;
    }
}

#[named]
fn part1_recursive_over_all_valves() {
    let now = Instant::now();

    let mut valves = load_data("src\\d16\\data_test.txt");
    reduce_graph(&mut valves);
    print_valves(&valves);
    best_result_from_recursive_over_all_valves("".to_string(), &mut valves, "AA", "", 30);
    println!(
        "{}: ({} sec)",
        function_name!(),
        now.elapsed().as_secs_f32()
    );
}

fn best_result_from(
    indent_string: String,
    valves: &mut HashMap<String, Valve>,
    distances: &Vec<Vec<i32>>,
    valves_opened_mask: u64,
    all_valves_opened_mask: u64,
    current_valve_index: i32,
    mut remaining_time: i32,
    best_for_valve_mask: &mut HashMap<u64, i32>,
) -> i32 {
    let current_valve = valves.values().nth(current_valve_index as usize).unwrap();
    if current_valve.rate > 0 {
        remaining_time -= 1;
    }
    if remaining_time <= 0 || valves_opened_mask == all_valves_opened_mask {
        0
    } else {
        let mut best: (i32, i32) = (0, 0); // (best result, best next valve index)
        if valves_opened_mask & (1 << current_valve_index) == 0 {
            let from_this_valve = current_valve.rate * remaining_time;
            for (next_valve_index, distance) in distances[current_valve_index as usize].iter().enumerate() {
                if current_valve_index != next_valve_index as i32 {
                    let result = best_result_from(
                            indent_string.clone()
                                + valves.keys().nth(current_valve_index as usize).unwrap()
                                + " -> ",
                            valves,
                            distances,
                            valves_opened_mask | 1 << current_valve_index,
                            all_valves_opened_mask,
                            next_valve_index as i32,
                            remaining_time - distance,
                            best_for_valve_mask
                        );
                    // if indent_string.len() < 30 {
                    //     println!("({}): {}{}: {} by going to {}", remaining_time, indent_string, valves.keys().nth(current_valve_index as usize).unwrap(), result, valves.keys().nth(next_valve_index as usize).unwrap());
                    // }
                    if result == 0 {
                        best_for_valve_mask.entry(valves_opened_mask).and_modify(|e| *e = (*e).max(2)).or_insert(result);
                    }
                    if result > best.0 {
                        best = (result, next_valve_index as i32);
                    }
                }
            }
            best.0 += from_this_valve;
        }
        if indent_string.len() < 2 {
            println!(
                "({}): {}{} best: {} by going to {}",
                remaining_time,
                indent_string,
                valves.keys().nth(current_valve_index as usize).unwrap(),
                best.0,
                valves.keys().nth(best.1 as usize).unwrap()
            );
        }
        best.0
    }
}

// #[named]
// fn part1() {
//     let now = Instant::now();
//     let mut valves = load_data("src\\d16\\data.txt");
//     reduce_graph(&mut valves);
//     print_valves(&valves);
//     let (distances, name_to_index) = floyd_warshall(&valves);
//     let mut valves_opened_mask = 0;
//     let mut best_for_valve_mask: HashMap<u64, i32> = HashMap::new();
//     let best = best_result_from(
//         "".to_string(),
//         &mut valves,
//         &distances,
//         &mut valves_opened_mask,
//         *name_to_index.get("AA").unwrap(),
//         30,
//         best_for_valve_mask,
//     );
//     println!(
//         "{}: {} ({} sec)",
//         function_name!(),
//         best,
//         now.elapsed().as_secs_f32()
//     );
// }

#[named]
fn part2() {
    let now = Instant::now();
    let mut valves = load_data("src\\d16\\data.txt");
    reduce_graph(&mut valves);
    print_valves(&valves);
    let (distances, name_to_index) = floyd_warshall(&valves);
    let start_index = *name_to_index.get("AA").unwrap();
    let all_valves_opened_mask = !(1 << start_index);
    let mut best_for_valve_mask: HashMap<u64, i32> = HashMap::new();
    let best = best_result_from(
        "".to_string(),
        &mut valves,
        &distances,
        0,
        all_valves_opened_mask,
        start_index,
        30,
        &mut best_for_valve_mask,
    );
    println!(
        "{}: {} ({} sec)",
        function_name!(),
        best,
        now.elapsed().as_secs_f32()
    );
}

pub fn run() {
    //part1_recursive_over_all_valves();
    //part1();
    part2();
}

// Valve AA (30): best: 1376 by leaving closed and going to HS
// part1: (1368.0825 sec)
// (30): AA best: 1376 by going to HS
// part1: 1376 (5.9803987 sec)
// part1: 1376 (5.4707456 sec, 0 combinations)
// part2: 1048 (1.3175429 sec, 0 combinations)
