use ::function_name::named;

const DAY: i32 = 11;

fn has_straight_of_3(input: &str) -> bool {
    let mut chars = input.chars();
    let mut prev_char = chars.next().unwrap();
    let mut prev_count = 1;
    for c in chars {
        if c == (prev_char as u8 + 1) as char {
            prev_count += 1;
            if prev_count == 3 {
                return true;
            }
        }
        else {
            prev_count = 1;
        }
        prev_char = c;
    }
    false
}

fn has_invalid_characters(input: &str) -> bool {
    input.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

fn contains_two_pairs(input: &str) -> bool {
    let mut chars = input.chars();
    let mut prev_char = chars.next().unwrap();
    let mut prev_count = 1;
    let mut pair_count = 0;
    for c in chars {
        if c == prev_char {
            prev_count += 1;
            if prev_count == 2 {
                pair_count += 1;
                prev_count = 0;
            }
        }
        else {
            prev_count = 1;
        }
        prev_char = c;
    }
    pair_count >= 2
}

fn increment_password(input: &str) -> String {
    let mut chars = input.chars().rev();
    let mut result = String::new();
    let mut carry = true;
    for c in chars {
        if carry {
            if c == 'z' {
                result.push('a');
            }
            else {
                result.push((c as u8 + 1) as char);
                carry = false;
            }
        }
        else {
            result.push(c);
        }
    }
    result.chars().rev().collect()
}

#[test]
fn test() {
    assert!(has_straight_of_3("hijklmmn") == true);
    assert!(has_straight_of_3("abbceffg") == false);
    assert!(has_invalid_characters("abbcegjk") == false);
    assert!(has_invalid_characters("abbcegik") == true);
    assert!(contains_two_pairs("abbceffg") == true);
    assert!(increment_password("abcdefgh") == "abcdefgi");
    assert!(increment_password("xx") == "xy");
    assert!(increment_password("xy") == "xz");
    assert!(increment_password("xz") == "ya");
}

#[named]
fn part1() {
    let mut res: String = "hepxcrrq".to_string();
    while !has_straight_of_3(&res) || has_invalid_characters(&res) || !contains_two_pairs(&res) {
        res = increment_password(&res);
    }
    println!("{}: {}", function_name!(), res);
}

#[named]
fn part2() {
    let mut res: String = increment_password("hepxxyzz");
    while !has_straight_of_3(&res) || has_invalid_characters(&res) || !contains_two_pairs(&res) {
        res = increment_password(&res);
    }
    println!("{}: {}", function_name!(), res);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 117
// part2: 909