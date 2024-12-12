use std::fs;

use ::function_name::named;

fn load_data(path: &str) -> Vec<(i64, Vec<i64>)> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let file_lines = file_contents_as_string.lines().collect::<Vec<&str>>();
    let mut ret = Vec::new();
    for line in file_lines {
        let mut parts = line.split(": ");
        let depth = parts.next().unwrap().parse().unwrap();
        let range = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();
        ret.push((depth, range));
    }
    ret
}

fn run_math_1(target: i64, current: i64, index: i32, nums: &Vec<i64>) -> bool {
    if index == nums.len() as i32 {
        return current == target;
    }
    run_math_1(target, current + nums[index as usize], index + 1, nums)
        || run_math_1(target, current * nums[index as usize], index + 1, nums)
}

fn run_math_2(target: i64, current: i64, index: i32, nums: &Vec<i64>) -> bool {
    if index == nums.len() as i32 {
        return current == target;
    }
    run_math_2(target, current + nums[index as usize], index + 1, nums)
        || run_math_2(target, current * nums[index as usize], index + 1, nums)
        || {
            let new_current = (current.to_string() + &nums[index as usize].to_string()).parse().unwrap();
            run_math_2(target, new_current, index + 1, nums)
        }
}

#[named]
fn part1() {
    let data = load_data("src\\d7\\data.txt");
    let sum = data
        .iter()
        .filter(|(target, nums)| run_math_1(*target, nums[0], 1, nums))
        .map(|(target, _)| target)
        .sum::<i64>();
    println!("{}: {}", function_name!(), sum);
}

#[named]
fn part2() {
    let data = load_data("src\\d7\\data.txt");
    let sum = data
        .iter()
        .filter(|(target, nums)| run_math_2(*target, nums[0], 1, nums))
        .map(|(target, _)| target)
        .sum::<i64>();
    println!("{}: {}", function_name!(), sum);
}

pub fn run() {
    part1();
    part2();
}

// part1: 945512582195
// part2: 271691107779347
