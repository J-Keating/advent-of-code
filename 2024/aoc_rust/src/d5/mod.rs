use std::fs;

use ::function_name::named;
use itertools::Itertools;
use utilities::alloc_2d_vec;

fn load_data(path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let (rules_string, page_lists_string) = file_contents_as_string.split("\r\n\r\n").collect_tuple().unwrap();
    let rules = rules_string.lines().map(|l| l.split("|").map(|s| s.parse().unwrap()).collect_tuple().unwrap()).collect();
    let page_lists = page_lists_string.lines().map(|l| l.split(",").map(|s| s.parse().unwrap()).collect()).collect();
    (rules, page_lists)
}

fn find_valid_page_lists(rules: &Vec<(i32, i32)>, page_lists: &Vec<Vec<i32>>, want_valid: bool) -> Vec<Vec<i32>> {
    page_lists.iter().filter(|page_list| {
        let mut valid = true;
        for i in 0..page_list.len()-1 {
            for j in i+1..page_list.len() {
                if rules.contains(&(page_list[j], page_list[i])) {
                    valid = false;
                    break;
                }
            }
        }
        valid == want_valid
    }).cloned().collect()
}

#[named]
fn part1() {
    let (rules, page_lists) = load_data("src\\d5\\data.txt");
    let valid_page_lists = find_valid_page_lists(&rules, &page_lists, true);
    let sum = valid_page_lists.iter().map(|page_list| page_list[(page_list.len()-1)/2]).sum::<i32>();
    println!("{}: {}", function_name!(), sum);
}

#[named]
fn part2() {
    let (rules, page_lists) = load_data("src\\d5\\data.txt");
    let invalid_page_lists = find_valid_page_lists(&rules, &page_lists, false);
    let sum = invalid_page_lists.iter().map(|page_list| page_list[(page_list.len()-1)/2]).sum::<i32>();
    println!("{}: {}", function_name!(), sum);
}

pub fn run() {
    part1();
    part2();
}

// part1: 5509
// part2: 
