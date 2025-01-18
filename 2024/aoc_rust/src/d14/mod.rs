use itertools::Itertools;
use num::signum;
use regex::Regex;
use std::fs;
use utilities::{Board, PointRC};
use ::function_name::named;

const DAY: &str = "d14";

mod test_data {
    pub const FILENAME: &str = r"src\d14\data_test.txt";
    pub const WIDTH: i32 = 11;
    pub const HEIGHT: i32 = 7;
}
mod real_data {
    pub const FILENAME: &str = r"src\d14\data.txt";
    pub const WIDTH: i32 = 101;
    pub const HEIGHT: i32 = 103;
}


struct Robot {
    loc: PointRC,
    vel: PointRC
}

fn load_data(path: &str) -> Vec<Robot> { 
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let lines = file_contents_as_string.lines().collect::<Vec<&str>>();
    lines.iter().map(|line| {
        let matches = re.captures(line).unwrap();
        Robot {
            loc: PointRC { r: matches[2].parse().unwrap(), c: matches[1].parse().unwrap() },
            vel: PointRC { r: matches[4].parse().unwrap(), c: matches[3].parse().unwrap() }
        }
    }).collect()
}

fn wrapping_mod(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}

#[test]
fn test_mod() {
    assert_eq!(wrapping_mod(3, 5), 3);
    assert_eq!(wrapping_mod(5, 3), 2);
    let x = wrapping_mod(-3, 5);
    assert_eq!(x, 2);
}

fn get_end_locs_at_time(robots: &Vec<Robot>, height: i32, width: i32, time: i32) -> Vec<PointRC> {
    let end_locs = robots.iter().map(|robot| robot.loc.add(&robot.vel.mul(time))).collect_vec();
    end_locs.iter().map(|loc| PointRC { r: wrapping_mod(loc.r, height), c: wrapping_mod(loc.c, width) }).collect()
}

fn count_quadrants(locs: &Vec<PointRC>, height: i32, width: i32) -> Vec<i32> {
    let end_locs = locs.iter().map(|loc| PointRC { r: loc.r - ((height - 1) / 2), c: loc.c - ((width - 1) / 2) }).collect_vec();
    
    let mut sector_counts = [0; 5];
    end_locs.iter().for_each(|loc| {
        let index = match (signum(loc.r), signum(loc.c)) {
            (-1, -1) => 0,
            (-1, 1) => 1,
            (1, -1) => 2,
            (1, 1) => 3,
            _ => 4
        };
        sector_counts[index] += 1;
    });
    sector_counts[..sector_counts.len() - 1].to_vec()
}

#[named]
fn part1() {
    use real_data as data;

    let robots = load_data(data::FILENAME);
    let end_locs = get_end_locs_at_time(&robots, data::HEIGHT, data::WIDTH, 100);
    let sector_counts = count_quadrants(&end_locs, data::HEIGHT, data::WIDTH);
    assert!(sector_counts.len() == 4);
    has_symmetry(&end_locs, data::HEIGHT, data::WIDTH);
    let factor = sector_counts.iter().fold(1, |acc, count| acc * count);
    println!("{}: {}", function_name!(), factor);
}

fn has_symmetry(locs: &Vec<PointRC>, height: i32, width: i32) -> bool {
    let mut board = Board::new(height as usize, width as usize, '.');
    locs.iter().for_each(|loc| board.data[loc.r as usize][loc.c as usize] = '#');
    for r in 0..board.height {
        for c in 0..((board.width - 1) / 2) {
            if board.data[r][c] != board.data[r][board.width - 1 - c] {
                println!("-----------------------------");
                board.print();
                return false;
            }
        }
    }
    true
}

fn print_as_board(locs: &Vec<PointRC>, height: i32, width: i32) {
    let mut board = Board::new(height as usize, width as usize, ' ');
    locs.iter().for_each(|loc| {
        board.data[loc.r as usize][loc.c as usize] = match board.data[loc.r as usize][loc.c as usize] {
            ' ' => '1',
            c => (c as u8 + 1) as char
        };
    });
    board.print();
}

#[named]
fn part2() {
    use real_data as data;

    let robots = load_data(data::FILENAME);
    for i in 0..10000000 {
        if i % 103 == 63 && i % 101 == 82 {
            let end_locs = get_end_locs_at_time(&robots, data::HEIGHT, data::WIDTH, i);
            println!("{} : -----------------------------", i);
            print_as_board(&end_locs, data::HEIGHT, data::WIDTH);
            break;
        }
    }
    println!("{}: {}", function_name!(), "Done");
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 222901875
// part2: 6243
