use ::function_name::named;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs, str::FromStr};

const DAY: i32 = 16;

lazy_static! {
    static ref INFO: HashMap<&'static str, i32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
}

fn load_data(filename: &str) -> String {
    let file_contents_as_string = fs::read_to_string(filename).unwrap();
    file_contents_as_string
}

fn part_1_compare(_: &str, a: i32, b: i32) -> bool {
    a == b
}

fn part_2_compare(animal: &str, a: i32, b: i32) -> bool {
    match animal {
        "cats" | "trees" => a < b,
        "pomeranians" | "goldfish" => a > b,
        _ => a == b,
    }
}

fn is_sue_a_match(data_string: &str, compare_func: fn(&str, i32, i32)->bool) -> bool {
    let re = Regex::new(r"Sue (\d+):(.+)").unwrap();
    for cap in re.captures_iter(data_string) {
        let _sue_number = i32::from_str(&cap[1]).unwrap();
        for item in cap[2].split(",") {
            let mut parts = item.split(":");
            let key = parts.next().unwrap().trim();
            let value = i32::from_str(parts.next().unwrap().trim()).unwrap();
            assert!(INFO.contains_key(key));
            if !compare_func(key, *INFO.get(key).unwrap(), value) {
                return false;
            }
        }
        //println!("Matching Sue = {}", sue_number);
        return true;
    }
    false
}

#[named]
fn part1() {
    let data = load_data(&format!("src\\d{}\\data.txt", DAY));
    for line in data.lines() {
        if is_sue_a_match(line, part_1_compare) {
            println!("{}: {}", function_name!(), line);
        }
    }
}

#[named]
fn part2() {
    let data = load_data(&format!("src\\d{}\\data.txt", DAY));
    for line in data.lines() {
        if is_sue_a_match(line, part_2_compare) {
            println!("{}: {}", function_name!(), line);
        }
    }
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1_test: 62842880
// part1: 222870
// part2: 117936
