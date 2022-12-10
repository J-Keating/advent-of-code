use std::{fs, io::BufRead};
//use std::str::FromStr;

use ::function_name::named;

#[named]
fn part1() {
    let mut score: i32 = 0;
    let file_contents = fs::read("src\\d2\\data.txt").expect("Error loading file");
    for line in file_contents.lines().map(|line| line.unwrap()) {
        let buff = line.as_bytes();
        assert!(buff.len() == 3 && buff[1] as char == ' ');
        let them = buff[0] as i32 - 'A' as i32;
        let me = buff[2] as i32 - 'X' as i32;
        if them == me {
            score += 3;
        }
        else if (them + 1) % 3 == me {
            score += 6;
        }
        score += me + 1
    }
    println!("{}: {}", function_name!(), score);
}

#[named]
fn part2() {
    let mut score: i32 = 0;
    let file_contents = fs::read_to_string("src\\d2\\data.txt").expect("Error loading file");
    for line in file_contents.split("\n").map(|x| x.trim()) {
        let plays: Vec<i32> = line.split(" ").map(|x| x.chars().nth(0).expect("char!") as i32).collect();
        assert!(plays.len() == 2);
        let them = plays[0] - 'A' as i32;
        let result = plays[1] - 'X' as i32;
        let me: i32 = match result {
            0 => { (them + 2) % 3 } // lose
            1 => { (them + 0) % 3 } // tie
            2 => { (them + 1) % 3 } // win
            _ => { panic!("Shouldn't be here!") }
        };
        score += me + 1 + (3 * result);
    }
    println!("{}: {}", function_name!(), score);
}

fn test() {
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let short_digits = digits
                        .iter()
                        .enumerate()
                        .filter(|&(index, &digit)| digit.len() < index)
                        .map(|(_, &digit)| digit);

    println!("Short digits:");
    for d in short_digits {
        println!("The word {} is shorter than its value.", d);
    }
}

fn test2() {
    let a = ["1", "two", "NaN", "four", "5"];
    {
        let mut iter = a.iter().map(|s| s.parse()).filter(|s| s.is_ok()).map(|s| s.unwrap());
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }
    {
        let mut iter = a.iter().filter_map(|s| s.parse().ok());

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);    }
}

pub fn run() {
    part1();
    part2();
    test();
    test2();
}

// part1: 12740
// part2: 11980
 