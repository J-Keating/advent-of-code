use ::function_name::named;

const DAY: i32 = 10;

fn look_and_say(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    let mut current_char = chars.next().unwrap();
    let mut current_count = 1;
    for c in chars {
        if c == current_char {
            current_count += 1;
        }
        else {
            result.push_str(&format!("{}{}", current_count, current_char));
            current_char = c;
            current_count = 1;
        }
    }
    result.push_str(&format!("{}{}", current_count, current_char));
    result
}

#[test]
fn test() {
    assert!(look_and_say("1") == "11");
    assert!(look_and_say("11") == "21");
    assert!(look_and_say("21") == "1211");
    assert!(look_and_say("1211") == "111221");
    assert!(look_and_say("111221") == "312211");
}

#[named]
fn part1() {
    let mut res: String = "1113222113".to_string();
    for _ in 0..40 {
        res = look_and_say(&res);
    }
    println!("{}: {}", function_name!(), res.len());
}

#[named]
fn part2() {
    let mut res: String = "1113222113".to_string();
    for _ in 0..50 {
        res = look_and_say(&res);
    }
    println!("{}: {}", function_name!(), res.len());
}

pub fn run() {
    println!("Day {}:", DAY);
    part1();
    part2();
}

// part1: 117
// part2: 909