use std::{fs, io::BufRead};
//use std::str::FromStr;

use ::function_name::named;

fn find_common2(s1: &str, s2: &str) -> char {
    for c in s1.chars() {
        if s2.find(c).is_some() {
            return c;
        }
    }
    panic!()
}

fn find_common3(s1: &str, s2: &str, s3: &str) -> char {
    for c in s1.chars() {
        if s2.find(c).is_some() && s3.find(c).is_some() {
            return c;
        }
    }
    panic!()
}

fn compute_score(c: char) -> i32 {
    if c.is_lowercase() {
        return c as i32 - 'a' as i32 + 1;
    }
    else if c.is_uppercase() {
        return c as i32 - 'A' as i32 + 27;
    }
    panic!()
}

#[named]
fn part1() {
    let mut score: i32 = 0;
    let file_contents = fs::read("src\\d3\\data.txt").expect("Error loading file");
    for line in file_contents.lines().map(|line| line.unwrap()) {
        let len = line.len();
        assert!(len %2 == 0);
        let (s1, s2) = line.split_at(len/2);
        let c = find_common2(s1, s2);
        score += compute_score(c);
    }
    println!("{}: {}", function_name!(), score);
}

#[named]
fn part2() {
    let mut score: i32 = 0;
    let file_contents = fs::read("src\\d3\\data.txt").expect("Error loading file");
    // let lines = file_contents.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    // assert!(lines.len() % 3 == 0);
    // let mut i = 0;
    // let len = lines.len();
    // while i < len {
    //     let c = find_common3(lines[i].as_str(), lines[i+1].as_str(), lines[i+2].as_str());
    let mut iter = file_contents.lines().map(|line| line.unwrap()).peekable();
    while iter.peek() != None {
        let c = find_common3(iter.next().unwrap().as_str(), iter.next().unwrap().as_str(), iter.next().unwrap().as_str());
        score += compute_score(c);
    }
    println!("{}: {}", function_name!(), score);
}

pub fn run() {
    part1();
    part2();
}

// part1: 8053
// part2: 2425
 