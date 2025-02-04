use function_name::named;
use itertools::Itertools;
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

#[named]
fn part1() {
    use test_data as data;
    let (towels, desired_designs) = load_data(data::FILENAME);
    println!("{}: {:?}, {:?}", function_name!(), towels, desired_designs);
}

#[named]
fn part2() {
    use real_data as data;
    let (towels, desired_designs) = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), towels.len() + desired_designs.len());
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 372
// part2: 25,6
