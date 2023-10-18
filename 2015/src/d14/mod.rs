use ::function_name::named;
use std::{fs, str::FromStr};
use regex::Regex;

const DAY: i32 = 14;

#[derive(Debug, Clone)]
struct ReindeerMovement {
    name: String,
    speed: i32,
    duration: i32,
    rest: i32,
}

fn load_data(filename: &str) -> Vec<ReindeerMovement> {
    let data = fs::read_to_string(filename).unwrap();
    let mut reindeers = Vec::new();
    let re = Regex::new(r"(.+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
        
    for line in data.lines() {
        for cap in re.captures_iter(line) {
            let name: String = cap[1].to_string();
            let speed = i32::from_str(&cap[2]).unwrap();
            let duration = i32::from_str(&cap[3]).unwrap();
            let rest = i32::from_str(&cap[4]).unwrap();
            reindeers.push(ReindeerMovement { name, speed, duration, rest });
        }
    }
    reindeers
}

fn get_distance(reindeer: &ReindeerMovement, time: i32) -> i32 {
    let cycle_time = reindeer.duration + reindeer.rest;
    let cycle_distance = reindeer.speed * reindeer.duration;
    let cycle_count = time / cycle_time;
    let remainder = time % cycle_time;
    let remainder_distance = if remainder > reindeer.duration {
        cycle_distance
    } else {
        remainder * reindeer.speed
    };
    cycle_count * cycle_distance + remainder_distance
}

fn find_winning_reindeer(reindeers: &Vec<ReindeerMovement>, time: i32) -> (i32, i32) {
    let mut max_distance = 0;
    let mut max_index: i32 = -1;
    for (i, reindeer)  in reindeers.iter().enumerate() {
        let distance = get_distance(reindeer, time);
        if distance > max_distance {
            max_distance = distance;
            max_index = i as i32;
        }
    }
    (max_index, max_distance)
}

#[named]
fn part1() {
    let reindeers = load_data(&format!("src\\d{}\\data.txt", DAY));
    let (index, max_distance) = find_winning_reindeer(&reindeers, 2503);
    println!("{}: {} went {} km", function_name!(), reindeers[index as usize].name, max_distance);
}

#[named]
fn part2() {
    let reindeers = load_data(&format!("src\\d{}\\data.txt", DAY));
    let points = &mut vec![0; reindeers.len()];
    for t in 1..=2503 {
        let (index, _) = find_winning_reindeer(&reindeers, t);
        points[index as usize] += 1;
    }
    let max_index = (0..points.len()).max_by_key(|&i| points[i]).unwrap();
    println!("{}: {} has {} points", function_name!(), reindeers[max_index].name, points[max_index]);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: Donner went 2655 km
// part2: 909