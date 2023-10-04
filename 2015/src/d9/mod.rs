use ::function_name::named;
use regex::Regex;
use std::{collections::HashMap, collections::HashSet, fs, vec};
use utilities::alloc_2d_vec;

const DAY: i32 = 9;

fn load_data(filename: &str) -> Vec<Vec<u16>> {
    let re = Regex::new(r"(.*)\s+to\s+(.*)\s+=\s+(.*)").unwrap();
    let data = fs::read_to_string(filename).unwrap();
    let mut city_set: HashSet<String> = HashSet::new();
    for line in data.lines() {
        for cap in re.captures_iter(line) {
            city_set.insert(cap[1].to_string());
            city_set.insert(cap[2].to_string());
        }
    }
    let mut name_to_index: HashMap<String, u16> = HashMap::new();
    for (i, name) in city_set.iter().enumerate() {
        name_to_index.insert(name.to_string(), i as u16);
    }
    let city_count = name_to_index.len();
    let mut ret = alloc_2d_vec(city_count, city_count, 0);
    for line in data.lines() {
        for cap in re.captures_iter(line) {
            let city1_idx = name_to_index.get(&cap[1]).unwrap();
            let city2_idx = name_to_index.get(&cap[2]).unwrap();
            let distance = cap[3].parse::<u16>().unwrap();
            ret[*city1_idx as usize][*city2_idx as usize] = distance;
            ret[*city2_idx as usize][*city1_idx as usize] = distance;
        }
    }
    // println!("{:?}", name_to_index);
    // ret.iter().for_each(|f| println!("{:?}", f));
    ret
}

fn move_to_city(
    visited: &mut Vec<bool>,
    current_city: usize,
    next_city: usize,
    mut total_distance: u16,
    distances: &Vec<Vec<u16>>,
    min_distance: &mut u16,
    max_distance: &mut u16,
) {
    total_distance += distances[current_city][next_city];
    visited[next_city] = true;
    
    if visited.iter().all(|&x| x) {
        if total_distance < *min_distance {
            *min_distance = total_distance;
        }
        if total_distance > *max_distance {
            *max_distance = total_distance;
        }
    }
    else {
        for i in 0..distances.len() {
            if i != next_city && !visited[i] {
                move_to_city(visited, next_city, i, total_distance, distances, min_distance, max_distance);
            }
        }
    }
    visited[next_city] = false;
}

fn find_shortest_and_longest_paths(distances: &Vec<Vec<u16>>) -> (u16, u16) {
    let mut min_distance = u16::MAX;
    let mut max_distance: u16 = u16::MIN;
    for i in 0..distances.len() {
        let mut cities = vec![false; distances.len()];
        move_to_city(
            &mut cities,
            i,
            i,
            0,
            distances,
            &mut min_distance,
            &mut max_distance,
        );
    }
    (min_distance, max_distance)
}

#[named]
fn part1() {
    let distances = load_data(&format!("src\\d{}\\data.txt", DAY));
    let (min_distance, _) = find_shortest_and_longest_paths(&distances);
    println!("{}: {}", function_name!(), min_distance);
}

#[named]
fn part2() {
    let distances = load_data(&format!("src\\d{}\\data.txt", DAY));
    let (_, max_distance) = find_shortest_and_longest_paths(&distances);
    println!("{}: {}", function_name!(), max_distance);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 117
// part2: 909