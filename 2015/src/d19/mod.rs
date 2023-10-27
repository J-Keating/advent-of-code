use ::function_name::named;
use itertools::{Itertools, iterate};
use std::{fs, collections::HashSet, collections::HashMap};

const DAY: i32 = 19;

#[derive(Clone)]
struct Replacement {
    from: String,
    to: String
}

fn load_data(filename: &str) -> (Vec<Replacement>, String) {
    let file_contents_as_string = fs::read_to_string(filename).unwrap();
    let (replacement_lines, start_string) = file_contents_as_string.split("\r\n\r\n").collect_tuple().unwrap();
    let replacements = replacement_lines.lines().map(|line| {
        let (from, to) = line.split(" => ").collect_tuple().unwrap();
        Replacement { from: from.to_string(), to: to.to_string() }
    }).collect::<Vec<Replacement>>();
    (replacements, start_string.to_string())
}

#[named]
fn part1() {
    let (replacements, start_string) = load_data(&format!("src\\d{}\\data.txt", DAY));
    let mut new_combinations: HashSet<String> = HashSet::new();
    for replacement in replacements.iter() {
        for (index, _) in start_string.match_indices(&replacement.from) {
            let new_combination = start_string[..index].to_string() + &replacement.to + &start_string[index+replacement.from.len()..];
            //println!("{}: {}", index, new_combination);
            new_combinations.insert(new_combination);
        }
    }
    println!("{}: {}", function_name!(), new_combinations.len());
}

#[allow(dead_code)]
#[named]
fn part2_no_good() {
    let (replacements, start_string) = load_data(&format!("src\\d{}\\data.txt", DAY));
    let mut replacement_count = 0;
    let mut new_combinations: HashSet<String> = HashSet::new();
    new_combinations.insert(start_string);
    while !new_combinations.contains("e") {
        let mut new_new_combinations: HashSet<String> = HashSet::new();
        for key in new_combinations.iter() {
            for replacement in replacements.iter() {
                for (index, _) in key.match_indices(&replacement.to) {
                    let new_combination = key[..index].to_string() + &replacement.from + &key[index+replacement.to.len()..];
                    new_new_combinations.insert(new_combination);
                }
            }
        }
        new_combinations = new_new_combinations;
        replacement_count += 1;
        //println!("{:?}", new_combinations);
    }
    println!("{}: {}", function_name!(), replacement_count);
}

#[named]
fn collapse(molecule: &str, replacements: &Vec<Replacement>, current_replacement_count: i32, min_replacement_count: &mut i32) {
    if current_replacement_count >= *min_replacement_count {
        return;
    }
    if molecule == "e" {
        println!("Made one! {} -> {}", *min_replacement_count, current_replacement_count);
        *min_replacement_count = current_replacement_count;
        return;
    }
    for replacement in replacements.iter() {
        for (index, _) in molecule.match_indices(&replacement.to) {
            let new_combination = molecule[..index].to_string() + &replacement.from + &molecule[index+replacement.to.len()..];
            collapse(&new_combination, replacements, current_replacement_count + 1, min_replacement_count);
        }
    }
}

#[named]
fn part2() {
    let (mut replacements, start_string) = load_data(&format!("src\\d{}\\data.txt", DAY));
    replacements.sort_by(|a, b| b.to.len().cmp(&a.to.len()));
    let mut min_replacement_count = 3;
    collapse(&start_string, &replacements, 0, &mut min_replacement_count);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 821
// part2: 886
 