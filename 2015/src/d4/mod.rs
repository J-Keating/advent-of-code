use std::{time::Instant};
use md5::{Md5, Digest};
use ::function_name::named;

type HashResultType = crypto_common::generic_array::GenericArray<u8, crypto_common::typenum::UInt<crypto_common::typenum::UInt<crypto_common::typenum::UInt<crypto_common::typenum::UInt<crypto_common::typenum::UInt<crypto_common::typenum::UTerm, crypto_common::typenum::B1>, crypto_common::typenum::B0>, crypto_common::typenum::B0>, crypto_common::typenum::B0>, crypto_common::typenum::B0>>;

fn find_first_key(prefix: &str, filter_function: fn(&HashResultType)->bool) -> u32 {
    let mut hasher = Md5::new();
    for i in 0.. {
        hasher.update(format!("{}{}", prefix, i).as_bytes());
        if filter_function(&hasher.finalize_reset()) {
            return i;
        }
    }
    panic!()
}

#[allow(dead_code)]
fn find_first_key_2(prefix: &str, filter_function: fn(&HashResultType)->bool) -> u32 {
    let mut hasher = Md5::new();
    itertools::iterate(0, |i| i + 1)
        .filter(|i| {
            hasher.update(format!("{}{}", prefix, i).as_bytes());
            filter_function(&hasher.finalize_reset())
        })
        .take(1)
        .last()
        .unwrap()
}

#[named]
fn part1() {
    // Test 
    // println!("{} (test): {}", function_name!(), find_first_key("abcdef"));
    // println!("{} (test): {}", function_name!(), find_first_key("pqrstuv"));
    let now = Instant::now();
    println!("{}: {} ({} sec)", function_name!(), find_first_key("bgvyzdsv", |x| x[0] == 0 && x[1] == 0 && x[2] < 16), now.elapsed().as_millis() as f32 / 1000.0);
    // now = Instant::now();
    // println!("{}: {} ({} sec)", function_name!(), find_first_key_2("bgvyzdsv", |x| x[0] == 0 && x[1] == 0 && x[2] < 16), now.elapsed().as_millis() as f32 / 1000.0);
}

#[named]
fn part2() {
    let now = Instant::now();
    println!("{}: {} ({} sec)", function_name!(), find_first_key("bgvyzdsv", |x| x[0] == 0 && x[1] == 0 && x[2] == 0), now.elapsed().as_millis() as f32 / 1000.0);
    // now = Instant::now();
    // println!("{}: {} ({} sec)", function_name!(), find_first_key_2("bgvyzdsv", |x| x[0] == 0 && x[1] == 0 && x[2] == 0), now.elapsed().as_millis() as f32 / 1000.0);
}

pub fn run() {
    println!("Day4:");
    part1();
    part2();
}

// Day4:
// part1: 254575 (1.185 sec)
// part1: 254575 (1.207 sec)
// part2: 1038736 (4.832 sec)
// part2: 1038736 (4.906 sec)
 