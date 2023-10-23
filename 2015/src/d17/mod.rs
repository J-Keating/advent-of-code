use std::{collections::HashSet, fs, str::FromStr};
use bit_field::BitField;

const DAY: i32 = 17;

fn load_data(filename: &str) -> Vec<i32> {
    let file_contents_as_string = fs::read_to_string(filename).unwrap();
    file_contents_as_string.lines().map(|x| i32::from_str(x).unwrap()).collect()
}

fn find_sum_to(data: &Vec<i32>, visited: &mut u64, current: i32, target: i32, combinations: &mut HashSet<u64>) {
    if current == target {
        combinations.insert(*visited);
        return;
    }
    if current > target {
        return;
    }
    for (i, item) in data.iter().enumerate() {
        if (*visited).get_bit(i) {
            continue;
        }
        (*visited).set_bit(i, true);
        find_sum_to(data, visited, current + item, target, combinations);
        (*visited).set_bit(i, false);
    }
}

pub fn run() {
    println!("Day {}:", DAY);

    let data = load_data(&format!("src\\d{}\\data.txt", DAY));
    assert!(data.len() <= 64);
    let mut visited: u64 = 0;
    let mut combinations = HashSet::<u64>::new();
    find_sum_to(&data, &mut visited, 0, 150, &mut combinations);
    println!("part1: {}", combinations.len());

    let container_counts = combinations.iter().map(|f| f.count_ones()).collect::<Vec<u32>>();
    let min = container_counts.iter().min().unwrap();
    let min_count = container_counts.iter().filter(|f| *f == min).count();
    println!("part2: {}", min_count);
}

// part1: 654
// part2: 57
