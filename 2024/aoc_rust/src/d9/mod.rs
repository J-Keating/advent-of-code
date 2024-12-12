use itertools::Itertools;
use std::fs;
use std::collections::{HashMap, HashSet};
use utilities::{Board, PointRC};
use ::function_name::named;

const DAY: &str = "d9";

#[named]
fn part1() {
    let file_contents_as_string = fs::read_to_string(&("src\\".to_string() + DAY + "\\data_test.txt")).expect("Error loading file");
    println!("{}: {}", function_name!(), file_contents_as_string);
}

#[named]
fn part2() {
    let file_contents_as_string = fs::read_to_string(&("src\\".to_string() + DAY + "\\data_test.txt")).expect("Error loading file");
    println!("{}: {}", function_name!(), file_contents_as_string);
}

pub fn run() {
    part1();
    part2();
}

// part1: 327
// part2: 1233
