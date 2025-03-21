//use itertools::Itertools;
use std::{fs, mem::swap};
//use std::collections::{HashMap, HashSet};
//use utilities::{Board, PointRC};
use ::function_name::named;

const DAY: &str = "d9";

fn expand_disk(map: &str) -> Vec<Option<i32>> {
    let disk_size: u32 = map.chars().map(|c| c.to_digit(10).unwrap()).sum();
    assert!(disk_size < 500000);
    let mut buffer: Vec<Option<i32>> = vec![None; disk_size as usize];
    let mut i: usize = 0;
    let mut id_number = 0;
    let mut is_empty = false;
    map.chars().for_each(|c| {
        let count = c.to_digit(10).unwrap();
        let val = if is_empty { None } else { Some(id_number) };
        for _ in 0..count {
            buffer[i] = val;
            i += 1;
        }
        if !is_empty {
            id_number += 1;
        }
        is_empty = !is_empty;
    });
    assert!(i == disk_size as usize);
    buffer
}

#[allow(dead_code)]
fn print_disk(buffer: &Vec<Option<i32>>) {
    buffer.iter().for_each(|&x| match x {
        Some(x) => print!("{:x}", x),
        None => print!("."),
    });
    println!();
}

#[named]
#[allow(dead_code)]
fn compute_checksum_old(buffer: &[Option<i32>]) -> i64 {
    let mut checksum = 0;
    for (i, val) in buffer.iter().enumerate() {
        if val.is_some() {
            checksum += i as i64 * val.unwrap() as i64;
        }
    }
    println!("{}: {}", function_name!(), checksum);
    checksum
}

fn compute_checksum(buffer: &[Option<i32>]) -> i64 {
    let checksum = buffer.iter().enumerate().filter(|(_, val)| { val.is_some() }).map(|(i, val)| { i as i64 * val.unwrap() as i64 }).sum();
    checksum
}

fn defrag_disk(map: &str) -> i64 {
    let mut buffer = expand_disk(map);
    //print_disk(&buffer);
    let mut start: usize = 0;
    let mut end: usize = buffer.len() - 1;
    loop {
        while buffer[start].is_some() {
            start += 1;
        }
        while buffer[end].is_none() {
            end -= 1;
        }
        if start >= end { break }
        let (b1, b2) = buffer.split_at_mut(end);
        swap::<Option<i32>>(&mut b1[start], &mut b2[0]);
    }
    //print_disk(&buffer);
    //compute_checksum(&buffer[0..start]);
    compute_checksum(&buffer[0..start])
}

#[named]
fn part1() {
    let file_contents_as_string = fs::read_to_string(&("src\\".to_string() + DAY + "\\data.txt")).expect("Error loading file");
    let checksum = defrag_disk(&file_contents_as_string);
    println!("{}: {}", function_name!(), checksum);
}

fn expand_disk_2(map: &str) -> Vec<(Option<i32>, usize)> {
    let buffer = map.chars().enumerate().map(|(i, c)| {
        let val = if i % 2 == 1 { None } else { Some(i as i32 / 2) };
        let count = c.to_digit(10).unwrap() as usize;
        (val, count)
    }).collect();
    buffer
}

#[allow(dead_code)]
fn print_disk_2(buffer: &Vec<(Option<i32>, usize)>) {
    buffer.iter().for_each(|(val, count)| {
        for _ in 0..*count {
            if val.is_some() {
                print!("{:x}", val.unwrap());
            } else {
                print!(".");
            }
        }
        print!("|");
    });
    println!();
}

fn compute_checksum_2(buffer: &Vec<(Option<i32>, usize)>) -> i64 {
    let mut index: usize = 0;
    buffer.iter().map(|(val, count)| {
        let run_sum = match val {
            Some(val) => (0..*count).into_iter().map(|i| {
                (index + i) as i64 * *val as i64
            }).sum(),
            None => 0,
        };
        index += *count;
        run_sum
    }).sum()
}

fn try_move_block(buffer: &mut Vec<(Option<i32>, usize)>, disk_id: i32) {
    let index = buffer.iter().position(|(val, _)| {
        val.is_some() && val.unwrap() == disk_id
    }).unwrap();
    let (block_val, block_size_needed) = buffer[index];
    let dest = buffer.iter().position(|(val, size)| {
        val.is_none() && *size >= block_size_needed
    });
    if dest.is_none() { return }
    let dest_index = dest.unwrap();
    if dest_index > index { return }
    // Empty space and coallesce
    buffer[index].0 = None;
    if index < buffer.len()-1 && buffer[index+1].0.is_none() {
        buffer[index].1 += buffer[index+1].1;
        buffer.remove(index+1);
    }
    if index > 0 && buffer[index-1].0.is_none() {
        buffer[index-1].1 += buffer[index].1;
        buffer.remove(index);
    }
    // Insert block
    buffer.insert(dest_index, (block_val, block_size_needed));
    buffer[dest_index+1].1 -= block_size_needed;
    if buffer[dest_index+1].1 == 0 {
        buffer.remove(dest_index+1);
    }
}

#[named]
fn part2() {
    let file_contents_as_string = fs::read_to_string(&("src\\".to_string() + DAY + "\\data.txt")).expect("Error loading file");
    let mut buffer2 = expand_disk_2(&file_contents_as_string);
    let _blocks_to_move = buffer2.iter().filter_map(|(val, _)| {
        *val
    }).collect::<Vec<i32>>();
    _blocks_to_move.iter().rev().for_each(|i| {
        try_move_block(&mut buffer2, *i);
    });
    let checksum2 = compute_checksum_2(&buffer2);
    
    println!("{}: {}", function_name!(), checksum2);
}

pub fn run() {
    part1();
    part2();
}

// part1: 6216544403458
// part2: 6237075041489
