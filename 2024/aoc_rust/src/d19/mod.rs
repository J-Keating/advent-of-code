use function_name::named;
use itertools::Itertools;
use memoize::memoize;
use std::{fs};

const DAY: &str = "d19";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d19\data_test.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d19\data.txt";
}

fn load_data(path: &str) -> (Vec<String>, Vec<String>) {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let (towels_string, desired_designs_string) = file_contents_as_string
        .split("\r\n\r\n")
        .collect_tuple()
        .unwrap();
    let towels = towels_string.split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>();
    let desired_designs = desired_designs_string
        .split("\r\n")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    (towels, desired_designs)
}

fn does_match(design: &str, towels: &Vec<String>) -> bool {
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) {
            if does_match(&design[towel.len()..], towels) {
                return true;
            }
        }
    }
    false
}

#[named]
fn part1() {
    use real_data as data;
    let (towels, desired_designs) = load_data(data::FILENAME);
    let count = desired_designs.iter().filter(|d| does_match(d, &towels)).count();
    println!("{}: {}", function_name!(), count);
}

#[memoize]
fn count_possible_matches(design: String, towels: Vec<String>) -> usize {
    if design.is_empty() {
        return 1;
    }
    let mut count = 0;
    for towel in &towels {
        if design.starts_with(towel.as_str()) {
            count += count_possible_matches(design[towel.len()..].to_string(), towels.clone());
        }
    }
    count
}

#[named]
fn part2() {
    use real_data as data;
    let (towels, desired_designs) = load_data(data::FILENAME);
    let count = desired_designs.iter().map(|d| count_possible_matches(d.clone(), towels.clone())).sum::<usize>();
    println!("{}: {}", function_name!(), count);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 315
// part2: 625108891232249
