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

fn load_data(path: &str) -> Vec<i64> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    file_contents_as_string.lines().map(|s| s.parse::<i64>().unwrap()).collect()
}

fn mix(secret: i64, given: i64) -> i64 {
    secret ^ given
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let mut next = prune(mix(secret, secret * 64));
    next = prune(mix(next, next / 32));
    prune(mix(next, next * 2048))
}

#[test]
fn test_stuff() {
    assert!(mix(42, 15) == 37);
    assert!(prune(100000000) == 16113920);
    assert!(next_secret(123) == 15887950);
    assert!(next_secret(15887950) == 16495136);
}

#[named]
fn part1() {
    use real_data as data;
    let numbers = load_data(data::FILENAME);
    let final_sum = numbers.iter().map(|&n| {
        let mut curr = n;
        for _ in 0..2000 {
            curr = next_secret(curr);
        }
        curr
    }).sum::<i64>();
    println!("{}: {:?}", function_name!(), final_sum);
}

#[named]
fn part2() {
    use test_data as data;
    let numbers: Vec<i64> = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), numbers.len());
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 315
// part2: 625108891232249
