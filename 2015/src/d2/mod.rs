use std::fs;
use std::str::FromStr;
use itertools::Itertools;

use ::function_name::named;

#[named]
fn part1() {
    let data = fs::read_to_string("src\\d2\\data.txt").unwrap();
    let res = data.lines().fold(0, |acc, line| {
        let sides = line.split('x').map(|s| u32::from_str(s).unwrap()).collect_vec();
        let faces_areas = sides.iter().combinations(2).map(|v| v[0] * v[1]).collect_vec();
        acc + faces_areas.iter().sum::<u32>() * 2 + faces_areas.iter().min().unwrap()
    });
    println!("{}: {}", function_name!(), res);
}

#[named]
fn part2() {
    let data = fs::read_to_string("src\\d2\\data.txt").unwrap();
    let res = data.lines().fold(0, |acc, line| {
        let sides = line.split('x').map(|s| u32::from_str(s).unwrap()).collect_vec();
        let min_permitter = sides.iter().combinations(2).map(|v| v[0] + v[1]).min().unwrap() * 2;
        acc + min_permitter + sides.iter().product::<u32>()
    });
    println!("{}: {}", function_name!(), res);
}

pub fn run() {
    part1();
    part2();
}

// part1: 1588178
// part2: 3783758
 