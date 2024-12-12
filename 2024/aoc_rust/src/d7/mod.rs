use std::{fs};

use ::function_name::named;

fn load_data(path: &str) -> Vec<(i64, Vec<i64>)> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let file_lines = file_contents_as_string.lines().collect::<Vec<&str>>();
    let mut ret = Vec::new();
    for line in file_lines {
        let mut parts = line.split(": ");
        let depth = parts.next().unwrap().parse().unwrap();
        let range = parts.next().unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        ret.push((depth, range));
    }
    ret
}

fn run_math(target: i64, current: i64, index: i32, nums: &Vec<i64>) -> i32{
    if index == nums.len() as i32 {
        return if current == target { 1 } else { 0 };
    }
    run_math(target, current + nums[index as usize], index + 1, nums) + run_math(target, current * nums[index as usize], index + 1, nums)
}

#[named]
fn part1() {
    let data = load_data("src\\d7\\data.txt");
    let mut sum = 0;
    for (target, nums) in data {
        let result = run_math(target, nums[0], 1, &nums);
        if result > 0 {
            sum += target;
        }
        //println!("{:?}: {}", nums, result);
    }
    println!("{}: {}", function_name!(), sum);
}

#[named]
fn part2() {
    let data = load_data("src\\d7\\data.txt");
    println!("{}: {}", function_name!(), data.len());
}

pub fn run() {
    part1();
    part2();
}

// part1: 945512582195
// part2: 850