use function_name::named;
//use itertools::Itertools;
use std::fs;
use utilities::{Board, PointRC};

const DAY: &str = "d20";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d20\data_test.txt";
    pub const DIMS: usize = 7;
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d20\data.txt";
    pub const DIMS: usize = 71;
}

fn load_data(path: &str) -> Vec<PointRC> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    file_contents_as_string
        .lines()
        .map(|s| {
            let mut parts = s.split(",");
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            PointRC { r: y, c: x }
        })
        .collect()
}

#[named]
fn part1() {
    use test_data as data;
    let points = load_data(data::FILENAME);
    let mut board = Board::<char>::new(data::DIMS, data::DIMS, '.');
    for p in &points[0..12] {
        board[*p] = '#';
    }
    board.print();
    println!("{}: {}", function_name!(), data::DIMS);
}

#[named]
fn part2() {
    use test_data as data;
    let _ = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), data::DIMS);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 2,1,0,1,7,2,5,0,3
// Found!: 267265166222235 (0o7461160522621633): 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0
