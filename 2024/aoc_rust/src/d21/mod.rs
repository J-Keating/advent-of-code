use function_name::named;
//use itertools::Itertools;
use std::fmt;
use utilities::{Board, PointRC};

const DAY: &str = "d21";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d21\data_test.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d21\data.txt";
}

fn load_data(path: &str) -> Vec<String> {
    std::fs::read_to_string(path).unwrap().lines().map(|s| s.to_string()).collect() 
}

#[named]
fn part1() {
    use test_data as data;
    let codes = load_data(data::FILENAME);
    println!("{}: {:?}", function_name!(), codes);
}

#[named]
fn part2() {
    use test_data as data;
    let codes = load_data(data::FILENAME);
    println!("{}: {:?}", function_name!(), codes);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 
// part2: 
