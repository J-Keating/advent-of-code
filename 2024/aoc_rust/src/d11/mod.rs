//use itertools::Itertools;
use std::fs;
//use std::collections::{HashMap, HashSet};
use ::function_name::named;
use itertools::Itertools;

const DAY: &str = "d11";

fn total_stones_after_blinks(num: i64, blinks_remaining: usize) -> i64 {
    if blinks_remaining == 0 {
        //println!("{}", num);
        return 1;
    }
    if num == 0 { return total_stones_after_blinks(1, blinks_remaining - 1); }
    let num_as_string = num.to_string();
    let num_len = num_as_string.len();
    if num_len % 2 == 0 {
        let (n1_as_string, n2_as_string) = num_as_string.split_at(num_len / 2);
        return total_stones_after_blinks(n1_as_string.parse().unwrap(), blinks_remaining - 1) +
        total_stones_after_blinks(n2_as_string.parse().unwrap(), blinks_remaining - 1) 
    }
    return total_stones_after_blinks(num * 2024, blinks_remaining - 1);
}

#[named]
fn part1() {
    let file_contents_as_string = fs::read_to_string(&("src\\".to_string() + DAY + "\\data_test.txt")).expect("Error loading file");
    let stones = file_contents_as_string.split(" ").map(|n| { n.parse::<i64>().unwrap() }).collect_vec();
    let total = stones.iter().map(|s| { total_stones_after_blinks(*s, 25) } ).sum::<i64>();
    println!("{}: {}", function_name!(), total);
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
