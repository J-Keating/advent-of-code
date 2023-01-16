use std::fs;
use ::function_name::named;

#[named]
fn part1() {
    let data = fs::read_to_string("src\\d1\\data.txt").unwrap();
    println!("{}: {}", function_name!(), data.matches('(').count() - data.matches(')').count());
}

#[named]
fn part2() {
    let data = fs::read_to_string("src\\d1\\data.txt").unwrap();
    let mut floor = 0;
    for (i, c) in data.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor < 0 {
            println!("{}: {}", function_name!(), i + 1);
            return;
        }
    }
}

pub fn run() {
    println!("Day1:");
    part1();
    part2();
}

// part1: 232
// part2: 1783
 