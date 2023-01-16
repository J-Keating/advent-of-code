use std::fs;
use ::function_name::named;

const DAY: i32 = 6;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const NAUGHTY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn test_string_part1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut double_letter = false;
    let mut naughty_string = false;
    for (i, c) in s.chars().enumerate() {
        if VOWELS.contains(&c) {
            vowel_count += 1;
        }
        if i < s.len() - 1 {
            if c == s.chars().nth(i + 1).unwrap() {
                double_letter = true;
            }
            if NAUGHTY_STRINGS.contains(&&s[i..i+2]) {
                naughty_string = true;
            }
        }
    }
    vowel_count >= 3 && double_letter && !naughty_string
}

fn test_string_part2(s: &str) -> bool {
    let mut double_pair = false;
    let mut sandwich = false;
    for (i, c) in s.chars().enumerate() {
        if i < s.len() - 2 {
            if s[i+2..].contains(&s[i..i+2]) {
                double_pair = true;
            }
            if c == s.chars().nth(i + 2).unwrap() {
                sandwich = true;
            }
        }
    }
    double_pair && sandwich
}

#[named]
fn part1() {
    // let data = fs::read_to_string(format!("src\\d{}\\data_test.txt", DAY)).unwrap();
    // for line in data.lines() {
    //     println!("{}: \"{}\": {}", function_name!(), line, test_string_part1(line));
    // }
    let data = fs::read_to_string(format!("src\\d{}\\data.txt", DAY)).unwrap();
    let count = data.lines().map(|l| test_string_part1(l)).filter(|b| *b).count();
    println!("{}: {}", function_name!(), count);
}

#[named]
fn part2() {
    let data = fs::read_to_string(format!("src\\d{}\\data.txt", DAY)).unwrap();
    // for line in data.lines() {
    //     println!("{}: \"{}\": {}", function_name!(), line, test_string_part2(line));
    // }
    let count = data.lines().map(|l| test_string_part2(l)).filter(|b| *b).count();
    println!("{}: {}", function_name!(), count);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 238
// part2: 69
 