use itertools::Itertools;
use num::signum;
use regex::Regex;
use std::fs;
use std::collections::{HashMap, HashSet};
use utilities::{Board, PointRC};
use ::function_name::named;

const DAY: &str = "d14";

mod test_data {
    pub const FILENAME: &str = r"src\d14\data_test.txt";
    pub const WIDTH: i32 = 11;
    pub const HEIGHT: i32 = 7;
}
mod real_data {
    pub const FILENAME: &str = r"src\d14\data.txt";
    pub const WIDTH: i32 = 101;
    pub const HEIGHT: i32 = 103;
}


struct Robot {
    loc: PointRC,
    vel: PointRC
}

fn load_data(path: &str) -> Vec<Robot> { 
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let lines = file_contents_as_string.lines().collect::<Vec<&str>>();
    lines.iter().map(|line| {
        let matches = re.captures(line).unwrap();
        Robot {
            loc: PointRC { r: matches[1].parse().unwrap(), c: matches[2].parse().unwrap() },
            vel: PointRC { r: matches[3].parse().unwrap(), c: matches[4].parse().unwrap() }
        }
    }).collect()
}

fn wrapping_mod(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}

#[test]
fn test_mod() {
    assert_eq!(wrapping_mod(3, 5), 3);
    assert_eq!(wrapping_mod(5, 3), 2);
    let x = wrapping_mod(-3, 5);
    assert_eq!(x, 2);
}

#[named]
fn part1() {
    use real_data as data;

    let robots = load_data(data::FILENAME);
    let mut end_locs = robots.iter().map(|robot| robot.loc.add(&robot.vel.mul(100))).collect_vec();
    end_locs = end_locs.iter().map(|loc| PointRC { r: wrapping_mod(loc.r, data::WIDTH), c: wrapping_mod(loc.c, data::HEIGHT) }).collect();
    end_locs = end_locs.iter().map(|loc| PointRC { r: loc.r - ((data::WIDTH - 1) / 2), c: loc.c - ((data::HEIGHT - 1) / 2) }).collect();
    
    let mut sector_counts = [0; 5];
    end_locs.iter().for_each(|loc| {
        let index = match (signum(loc.r), signum(loc.c)) {
            (1, 1) => 0,
            (1, -1) => 1,
            (-1, -1) => 2,
            (-1, 1) => 3,
            _ => 4
        };
        sector_counts[index] += 1;
    });
    let factor = sector_counts[..sector_counts.len() - 1].iter().fold(1, |acc, count| acc * count);
    println!("{}: {}", function_name!(), factor);
}

#[named]
fn part2() {
    let file_contents_as_string = fs::read_to_string(&("src\\".to_string() + DAY + "\\data_test.txt")).expect("Error loading file");
    println!("{}: {}", function_name!(), file_contents_as_string);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 327
// part2: 1233
