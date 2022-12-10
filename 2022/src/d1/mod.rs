use std::fs;
use std::str::FromStr;

use ::function_name::named;

#[named]
fn part1_v1() {
    let mut max_sum = 0;
    let mut cur_sum = 0;
    let file_contents = fs::read_to_string("src\\d1\\data.txt").expect("Error loading file");
    for line in file_contents.split("\n").map(|x| x.trim()) {
        if line.is_empty() {
            if cur_sum > max_sum {
                max_sum = cur_sum;
            }
            cur_sum = 0;
        }
        else {
            //cur_sum += i32::from_str(line).unwrap_or_else(|err| panic!("Could not parse {:?} as number, {}", line, err))
            cur_sum += line.parse::<i32>().ok().unwrap();
        }
    }
    println!("{}: {}", function_name!(), max_sum);
}

fn get_sums() -> Vec<i32> {
    let file_contents = fs::read_to_string("src\\d1\\data.txt").expect("Error loading file");
    let blocks = file_contents.split("\r\n\r\n");
    blocks.map(|block| {
        block.split("\r\n")
        .map(|line| {
            i32::from_str(line).unwrap_or_else(|err| panic!("Could not parse {:?} as number, {}", stringify!($line), err))
        })
        .fold(0, |sum, val| sum + val)
    }).collect::<Vec<i32>>()
}
 

#[named]
fn part1() {
    println!("{}: {}", function_name!(), get_sums().iter().max().unwrap());
}

#[named]
fn part2() {
    let mut sorted_sums = get_sums();
    sorted_sums.sort_by(|a,b| b.cmp(a));
    println!("{}: {}", function_name!(), sorted_sums.iter().take(3).fold(0, |sum, x| sum + x));
}

pub fn run() {
    part1_v1();
    part1();
    part2();
}

// part1: 68923
// part2: 200044
 