use num::abs;
use regex::Regex;
use std::fs;
use utilities::PointXY;
use ::function_name::named;

const DAY: &str = "d13";

struct Machine {
    button_a: PointXY::<i64>,
    button_b: PointXY::<i64>,
    prize: PointXY::<i64>
}

fn load_data(path: &str, offset: i64) -> Vec<Machine> { 
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let machine_strings = file_contents_as_string.split("\r\n\r\n").collect::<Vec<&str>>();
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)").unwrap();
    machine_strings.iter().map(|machine_string| {
        let matches = re.captures(machine_string).unwrap();
        Machine {
            button_a: PointXY { x: matches[1].parse().unwrap(), y: matches[2].parse().unwrap() },
            button_b: PointXY { x: matches[3].parse().unwrap(), y: matches[4].parse().unwrap() },
            prize: PointXY { x: matches[5].parse::<i64>().unwrap() + offset, y: matches[6].parse::<i64>().unwrap() + offset }
        }
   }).collect()
}

fn find_button_combo(machine: &Machine) -> Option<(i64, i64)> {
    let slope_a = (machine.button_a.y as f32) / (machine.button_a.x as f32);
    let slope_b = (machine.button_b.y as f32) / (machine.button_b.x as f32);
    assert!(abs(slope_a - slope_b) > 0.0001, "Slopes Equal: {} {}", slope_a, slope_b);

    for button_a_presses in 0..=100 {
        let move_a = machine.button_a.mul(button_a_presses as i64);
        let move_b = machine.prize.sub(&move_a);
        if move_b.x < 0 || move_b.y < 0 {
            return None;
        }
        if move_b.x % machine.button_b.x == 0 && move_b.y % machine.button_b.y == 0 {
            let button_b_combo = move_b.x / machine.button_b.x;
            if move_b.y / machine.button_b.y == button_b_combo {
                return Some((button_a_presses, button_b_combo as i64));
            }
        }
    }
    None
}

#[named]
fn part1() {
    let machines: Vec<Machine> = load_data(&("src\\".to_string() + DAY + "\\data.txt"), 0i64);
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

fn find_button_combo2(machine: &Machine) -> Option<(i64, i64)> {
    let m1 = (machine.button_a.y as f64) / (machine.button_a.x as f64);
    let m2 = (machine.button_b.y as f64) / (machine.button_b.x as f64);
    assert!(abs(m1 - m2) > 0.0001, "Slopes Equal: {} {}", m1, m2);
    
    let b2 = machine.prize.y as f64 - m2 * machine.prize.x as f64;
    let x = b2 / (m1 - m2);
    let y = m1 * x;
    if x >= 0.0 && y >= 0.0 {
        let intersection: PointXY<i64> = PointXY { x: x.round() as i64, y: y.round() as i64 };
        let button_a_presses_approx = intersection.x / machine.button_a.x;
        {
            for button_a_presses in button_a_presses_approx-2..=button_a_presses_approx+2 {
                let move_a = machine.button_a.mul(button_a_presses as i64);
                let move_b = machine.prize.sub(&move_a);
                if move_b.x < 0 || move_b.y < 0 {
                    return None;
                }
                if move_b.x % machine.button_b.x == 0 && move_b.y % machine.button_b.y == 0 {
                    let button_b_combo = move_b.x / machine.button_b.x;
                    if move_b.y / machine.button_b.y == button_b_combo {
                        return Some((button_a_presses, button_b_combo as i64));
                    }
                }
            }
        }
    }
    None
}

#[named]
fn part2() {
    let machines: Vec<Machine> = load_data(&("src\\".to_string() + DAY + "\\data.txt"), 10000000000000);
    let mut total = 0;
    for machine in machines {
        total += match find_button_combo2(&machine) {
            Some((a, b)) => a * 3 + b,
            None => 0
        };
    }
    println!("{}: {}", function_name!(), total);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 26810
// part2: 108713182988244
