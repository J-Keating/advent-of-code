use function_name::named;
//use itertools::Itertools;
use std::fs;

const DAY: &str = "d22";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d22\data_test.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d22\data.txt";
}

fn load_data(path: &str) -> Vec<String> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let lines = file_contents_as_string.lines().map(|s| s.to_string()).collect();
    lines
}

#[named]
fn part1() {
    use test_data as data;
    let lines = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), lines.len());
}

#[named]
fn part2() {
    use test_data as data;
    let lines = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), lines.len());
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 315
// part2: 625108891232249
