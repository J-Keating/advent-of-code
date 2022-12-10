use std::{fs, io::BufRead};

use ::function_name::named;
use ::itertools::Itertools;

#[named]
fn part1() {
    let mut score: i32 = 0;
    let file_contents = fs::read("src\\d4\\data.txt").expect("Error loading file");
    for line in file_contents.lines().map(|line| line.unwrap()) {
        let elves = line.split(&['-', ','][..]).collect::<Vec<&str>>();
        assert!(elves.len() == 4);
        let bounds = elves.iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if bounds[2] >= bounds[0] && bounds[3] <= bounds[1] {
            score += 1;
        }
        else if bounds[0] >= bounds[2] && bounds[1] <= bounds[3] {
            score += 1;
        }
    }
    println!("{}: {}", function_name!(), score);
}

#[named]
fn part2() {
    let mut score: i32 = 0;
    let file_contents = fs::read("src\\d4\\data.txt").expect("Error loading file");
    for line in file_contents.lines().map(|line| line.unwrap()) {
    let (start1, end1, start2, end2) = line.split(&['-', ','][..]).map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap();
        if !(start1 > end2 || start2 > end1) {
            score += 1;
        }
    }
    println!("{}: {}", function_name!(), score);
}

pub fn run() {
    part1();
    part2();
}

// part1: 511
// part2: 821 