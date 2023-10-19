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

fn is_sue_a_match(data_string: &str) -> bool {
    let re = Regex::new(r"Sue (\d+):(.+)").unwrap();
    for cap in re.captures_iter(data_string) {
        let sue_number = i32::from_str(&cap[1]).unwrap();
        for item in cap[2].split(",") {
            let mut parts = item.split(":");
            let key = parts.next().unwrap().trim();
            let value = i32::from_str(parts.next().unwrap().trim()).unwrap();
            assert!(INFO.contains_key(key));
            if INFO.get(key) != Some(&value) {
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
        if is_sue_a_match(line) {
            println!("{}: {}", function_name!(), line);
        }
    }
    println!("{}: {}", function_name!(), data.lines().count());
}

#[named]
fn part2() {
    println!("{}: {}", function_name!(), 0);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1_test: 62842880
// part1: 222870
// part2: 117936
