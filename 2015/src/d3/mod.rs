use std::collections::HashSet;
use std::fs;
use ::function_name::named;

fn move_in_direction(curr: (i32, i32), c: char) -> (i32, i32) {
    match c {
        '^' => (curr.0, curr.1 + 1),
        'v' => (curr.0, curr.1 - 1),
        '>' => (curr.0 + 1, curr.1),
        '<' => (curr.0 - 1, curr.1),
        _ => { panic!("Unknown direction") }
    }
}

#[named]
fn part1() {
    let data = fs::read_to_string("src\\d3\\data.txt").unwrap();
    for line in data.lines() {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut curr = (0, 0);
        visited.insert(curr);
        for c in line.chars().step_by(1) {
            curr = move_in_direction(curr, c);
            visited.insert(curr);
        }
        println!("{}: {}", function_name!(), visited.len());
    }
}

#[named]
fn part2() {
    let data = fs::read_to_string("src\\d3\\data.txt").unwrap();
    for line in data.lines() {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut curr = (0, 0);
        visited.insert(curr);
        for c in line.chars().step_by(2) {
            curr = move_in_direction(curr, c);
            visited.insert(curr);
        }
        curr = (0, 0);
        for c in line.chars().skip(1).step_by(2) {
            curr = move_in_direction(curr, c);
            visited.insert(curr);
        }
        println!("{}: {}", function_name!(), visited.len());
    }
}

pub fn run() {
    println!("Day3:");
    part1();
    part2();
}

// part1: 2572
// part2: 3783758
 