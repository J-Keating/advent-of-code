use std::{fs, collections::HashMap};

use ::function_name::named;

fn has_dupe(s: &str) -> bool {
    let mut counts: HashMap<char, bool>= HashMap::new();
    for c in s.chars() {
        if counts.contains_key(&c) {
            return true;
        }
        counts.insert(c, true);
    }
    false
}

#[test]
fn test_has_dupe() {
    assert!(has_dupe("abcdefg") == false);
    assert!(has_dupe("abcdeafg") == true);
}

#[named]
fn part1() {
    let file_contents = fs::read_to_string("src\\d6\\data.txt").expect("Error loading file");

    let mut match_pos = 0;
    let s = file_contents.lines().next().unwrap();
    let b = s.as_bytes();
    let l = s.len();
    for i in 3..l {
        if  (b[i-3] != b[i-2] && b[i-3] != b[i-1] && b[i-3] != b[i-0]) &&
            (b[i-2] != b[i-3] && b[i-2] != b[i-1] && b[i-2] != b[i-0]) &&
            (b[i-1] != b[i-3] && b[i-1] != b[i-2] && b[i-1] != b[i-0]) &&
            (b[i-0] != b[i-3] && b[i-0] != b[i-2] && b[i-0] != b[i-1]) {
                match_pos = i+1;
                break;
        }
    }

    println!("{}: {}", function_name!(), match_pos);
}

#[named]
fn part2() {
    let file_contents = fs::read_to_string("src\\d6\\data.txt").expect("Error loading file");

    let mut match_pos = 0;
    let s = file_contents.lines().next().unwrap();
    let size = 14;
    for i in size..s.len() {
        if !has_dupe(&s[i-size..i]) {
            match_pos = i;
            break;
        }
    }

    println!("{}: {}", function_name!(), match_pos)
}

pub fn run() {
    part1();
    part2();
}

// part1: 1707
// part2: 3697