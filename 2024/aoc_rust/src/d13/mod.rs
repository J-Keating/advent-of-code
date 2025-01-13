use num::abs;
use regex::Regex;
use std::fs;
use utilities::PointXY;
use ::function_name::named;

const DAY: &str = "d13";

struct Machine {
    button_a: PointXY,
    button_b: PointXY,
    prize: PointXY
}

fn load_data(path: &str) -> Vec<Machine> { 
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let machine_strings = file_contents_as_string.split("\r\n\r\n").collect::<Vec<&str>>();
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)").unwrap();
    machine_strings.iter().map(|machine_string| {
        let matches = re.captures(machine_string).unwrap();
        Machine {
            button_a: PointXY { x: matches[1].parse().unwrap(), y: matches[2].parse().unwrap() },
            button_b: PointXY { x: matches[3].parse().unwrap(), y: matches[4].parse().unwrap() },
            prize: PointXY { x: matches[5].parse().unwrap(), y: matches[6].parse().unwrap() }
        }
   }).collect()
}

fn find_button_combo(machine: &Machine) -> Option<(i32, i32)> {
    let slope_a = (machine.button_a.y as f32) / (machine.button_a.x as f32);
    let slope_b = (machine.button_b.y as f32) / (machine.button_b.x as f32);
    if abs(slope_a - slope_b) < 0.0001 {
        println!("Slopes Equal: {} {}", slope_a, slope_b);
    }
    for button_a_presses in 0..=100 {
        let move_a = machine.button_a.mul(button_a_presses);
        let move_b = machine.prize.sub(&move_a);
        if move_b.x < 0 || move_b.y < 0 {
            return None;
        }
        if move_b.x % machine.button_b.x == 0 && move_b.y % machine.button_b.y == 0 {
            let button_b_combo = move_b.x / machine.button_b.x;
            if move_b.y / machine.button_b.y == button_b_combo {
                return Some((button_a_presses, button_b_combo));
            }
        }
    }
    None
}

#[named]
fn part1() {
    let machines = load_data(&("src\\".to_string() + DAY + "\\data.txt"));
    //let total = machines.iter().map(|machine| find_button_combo(machine)).filter(|x| x.is_some()).map(|x| x.unwrap()).map(|(a, b)| a * 3 + b).sum::<i32>();
    let mut total = 0;
    for machine in machines {
        total += match find_button_combo(&machine) {
            Some((a, b)) => a * 3 + b,
            None => 0
        };
    }
    println!("{}: {}", function_name!(), total);
}

#[named]
fn part2() {
    let file_contents_as_string = fs::read_to_string(&("src\\".to_string() + DAY + "\\data_test.txt")).expect("Error loading file");
    println!("{}: {}", function_name!(), file_contents_as_string);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 327
// part2: 1233
