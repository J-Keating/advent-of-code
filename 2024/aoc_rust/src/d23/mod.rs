use function_name::named;
//use itertools::Itertools;
use std::fs;

const DAY: &str = "d23";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d23\data_test.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d23\data.txt";
}

type Edge = (i32, i32);

fn encode_computer(name: &str) -> i32 {
    assert!(name.len() == 2);
    (name.chars().nth(0).unwrap() as i32 - 'a' as i32) * 26 + (name.chars().nth(1).unwrap() as i32 - 'a' as i32)
}

fn decode_computer(code: i32) -> String {
    let a = (code / 26) as u8 + 'a' as u8;
    let b = (code % 26) as u8 + 'a' as u8;
    format!("{}{}", a as char, b as char)
}

#[test]
fn test_encode_computer() {
    assert_eq!(encode_computer("aa"), 0);
    assert_eq!(encode_computer("ab"), 1);
    assert_eq!(encode_computer("az"), 25);
    assert_eq!(encode_computer("ba"), 26);
    assert_eq!(decode_computer(0), "aa");
    assert_eq!(decode_computer(1), "ab");
    assert_eq!(decode_computer(26), "ba");
    assert!(decode_computer(encode_computer("zz")) == "zz");
    assert!(decode_computer(encode_computer("az")) == "az");
    assert!(decode_computer(encode_computer("za")) == "za");
    assert!(decode_computer(encode_computer("zy")) == "zy");
}

fn load_data(path: &str) -> Vec<Edge> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    file_contents_as_string.lines().map(|s| {
        let parts: Vec<_> = s.split('-').collect();
        assert!(parts.len() == 2);
        (encode_computer(parts[0]), encode_computer(parts[1]))
    }).collect()
}

#[named]
fn part1() {
    use test_data as data;
    let edges = load_data(data::FILENAME);
    println!("{}: {}, {:?}", function_name!(), edges.len(), edges[0..2].to_vec());
}

#[named]
fn part2() {
    use real_data as data;
    let edges = load_data(data::FILENAME);
    println!("{}: {}", function_name!(), edges.len());
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 315
// part2: 625108891232249
