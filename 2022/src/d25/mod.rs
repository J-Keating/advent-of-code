use std::{fs, time::Instant, collections::{HashSet, VecDeque}};

use ::function_name::named;

fn load_data(filename: &str) -> Vec<String> {
    fs::read_to_string(filename).unwrap().lines().map(|s| s.to_string()).collect()
}

const BASE: i64 = 5;
fn snafu_string_to_i64(snafu_string: &str) -> i64 {
    snafu_string.chars().rev().enumerate().map(|(i, c)| {
        let value = match c {
            '-' => -1,
            '=' => -2,
            '0'..='2' => c.to_digit(10).unwrap() as i64,
            _ => panic!("Invalid character")
        };
        let res = value * i64::pow(BASE, i as u32);
        //println!("{}", res);
        res
    }).sum::<i64>()
}

fn i64_to_snafu_string(decimal: i64) -> String {
    let mut char_buffer: VecDeque<char> = VecDeque::new();
    let mut decimal = decimal;
    while decimal != 0 {
        let digit = (decimal % BASE + 2) % BASE - 2;
        let c = match digit {
            -2 => '=',
            -1 => '-',
            0..=2 => (digit + '0' as i64) as u8 as char,
            _ => panic!("Invalid digit")
        };
        char_buffer.push_front(c);
        decimal = (decimal + 2) / BASE;
    }
    char_buffer.iter().collect()
}

#[test]
fn test_to_snafu() {
    for i in 0..100 {
        println!("{}: {}", i, i64_to_snafu_string(i));
    }
}

#[named]
fn part1() {
    let now = Instant::now();

    let snafu_strings = load_data("src\\d25\\data.txt");
    let total: i64 = snafu_strings.iter().map(|s| snafu_string_to_i64(&s)).sum();

    println!("{}: {} ({} ms)", function_name!(), i64_to_snafu_string(total), now.elapsed().as_micros() as f32 / 1000.0);
}

#[named]
pub fn run() {
    part1();
}

// part1: 2=0=02-0----2-=02-10 (1.546 ms)