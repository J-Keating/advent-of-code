use function_name::named;
//use itertools::Itertools;
use std::fmt;
use utilities::{Board, PointXY};

const DAY: &str = "d21";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d21\data_test.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d21\data.txt";
}

enum PadType {
    Number,
    Direction
}

fn num_to_loc(c: char) -> PointXY<i32> {
    match c {
        'A' => PointXY { x: 2, y: 0 },
        '0' => PointXY { x: 1, y: 0 },
        '1'..='9' => {
            let num = c.to_digit(10).unwrap() as i32 - 1;
            PointXY { x: num % 3, y: num / 3 + 1 }
        }
        _ => panic!("Unexpected char"),
    }
}

fn dir_to_loc(c: char) -> PointXY<i32> {
    match c {
        '<' => PointXY { x: 0, y: 0 },
        'v' => PointXY { x: 1, y: 0 },
        '>' => PointXY { x: 2, y: 0 },
        '^' => PointXY { x: 1, y: 1 },
        'A' => PointXY { x: 2, y: 1 },
        _ => panic!("Unexpected char"),
    }
}

#[test]
fn test_num_to_loc() {
    assert!(num_to_loc('A') == PointXY { x: 2, y: 0 });
    assert!(num_to_loc('0') == PointXY { x: 1, y: 0 });
    assert!(num_to_loc('1') == PointXY { x: 0, y: 1 });
    assert!(num_to_loc('2') == PointXY { x: 1, y: 1 });
    assert!(num_to_loc('9') == PointXY { x: 2, y: 3 });    
}

#[derive(Debug, PartialEq, Eq)]
struct Sequence {
    up: usize,
    down: usize,
    left: usize,
    right: usize
}    

impl Sequence {
    fn from_points(start: &PointXY<i32>, end: &PointXY<i32>) -> Sequence {
        let diff = end.sub(&start);
        let mut seq = Sequence { up: 0, down: 0, left: 0, right: 0 };
        match start.x.cmp(&end.x) {
            std::cmp::Ordering::Less => seq.right = diff.x as usize,
            std::cmp::Ordering::Greater => seq.left = num::abs(diff.x) as usize,
            _ => {},
        };    
        match start.y.cmp(&end.y) {
            std::cmp::Ordering::Less => seq.up = diff.y as usize,
            std::cmp::Ordering::Greater => seq.down = num::abs(diff.y) as usize,
            _ => {},
        };
        assert!((seq.up == 0 || seq.down == 0) && (seq.left == 0 || seq.right == 0), "Invalid sequence: {:?}", seq);
        seq
    }  

    fn to_dir_string(&self, start: &PointXY<i32>, pad_type: PadType) -> String {
        let mut s = String::new();
        //let mut front_load_left = true;
        let cant_do_left_first = match pad_type {
            PadType::Number => start.y == 0 && (start.x - self.left as i32) == 0,
            PadType::Direction => start.y == 1 && (start.x - self.left as i32) == 0,
        };
        let cant_do_right_last = match pad_type {
            PadType::Number => start.x == 0 && (start.y - self.down as i32) == 0,
            PadType::Direction => start.x == 0 && (start.y + self.up as i32) == 1,
        };
        if !cant_do_left_first {
            for _ in 0..self.left { s.push('<'); }
        }
        if cant_do_right_last {
            for _ in 0..self.right { s.push('>'); }
        }
        for _ in 0..self.up { s.push('^'); }
        for _ in 0..self.down { s.push('v'); }
        if !cant_do_right_last {
            for _ in 0..self.right { s.push('>'); }
        }
        if cant_do_left_first {
            for _ in 0..self.left { s.push('<'); }
        }
        s.push('A');
        s
    }
}

#[allow(dead_code)]
fn test_num_sequence_and_string(from: char, to: char, expected: Sequence, expected_string: &str) {
    let start: PointXY<i32> = num_to_loc(from);
    let end = num_to_loc(to);
    let actual = Sequence::from_points(&start, &end);
    assert!(actual == expected);
    assert!(actual.to_dir_string(&start, PadType::Number) == expected_string);
}

#[test]
fn test_num_sequences() {
    test_num_sequence_and_string('A', '7', Sequence { up: 3, down: 0, left: 2, right: 0 }, "^^^<<A");
    test_num_sequence_and_string('7', 'A', Sequence { up: 0, down: 3, left: 0, right: 2 }, ">>vvvA");
    test_num_sequence_and_string('1', '2', Sequence { up: 0, down: 0, left: 0, right: 1 }, ">A");
    test_num_sequence_and_string('4', '1', Sequence { up: 0, down: 1, left: 0, right: 0 }, "vA");
    test_num_sequence_and_string('5', '5', Sequence { up: 0, down: 0, left: 0, right: 0 }, "A");
}

#[test]
fn test_case_1() {
    let mut curr_string = "029A".to_string();
    let mut next_string = String::new();
    let mut prev_char = 'A';
    for c in curr_string.chars() {
        let start = num_to_loc(prev_char);
        let seq = Sequence::from_points(&start, &num_to_loc(c));
        //println!("{}: {:?}", seq.to_dir_string(), seq);
        next_string.push_str(&seq.to_dir_string(&start, PadType::Number));
        prev_char = c;
    }
    //println!("{}", next_string);
    assert!(next_string == "<A^A>^^AvvvA");
    curr_string = next_string;
    next_string = String::new();
    for c in curr_string.chars() {
        let start: PointXY<i32> = num_to_loc(prev_char);
        let seq = Sequence::from_points(&start, &dir_to_loc(c));
        //println!("{}: {:?}", seq.to_dir_string(), seq);
        next_string.push_str(&seq.to_dir_string(&start, PadType::Direction));
        prev_char = c;
    }
    let answer = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
    println!("{}", next_string);
    println!("{}", answer);
    assert!(next_string == answer);
}

fn through_all_keypads(code: &str) -> String {
    let mut curr_string = code.to_string();
    let mut next_string = String::new();
    let mut prev_char = 'A';
    for c in curr_string.chars() {
        let start: PointXY<i32> = num_to_loc(prev_char);
        let seq = Sequence::from_points(&start, &num_to_loc(c));
        //println!("{}: {:?}", seq.to_dir_string(), seq);
        next_string.push_str(&seq.to_dir_string(&start, PadType::Number));
        prev_char = c;
    }
    //println!("{}", next_string);
    for _ in 1..=2 {
        // println!("{}", next_string);
        curr_string = next_string;
        next_string = String::new();
        prev_char = 'A';
        for c in curr_string.chars() {
            let start: PointXY<i32> = dir_to_loc(prev_char);
            let seq = Sequence::from_points(&start, &dir_to_loc(c));
            //println!("{} - {}   {}: {:?}", prev_char, c, seq.to_dir_string(&start, PadType::Direction), seq);
            next_string.push_str(&seq.to_dir_string(&start, PadType::Direction));
            prev_char = c;
        }
        //println!("{}", next_string);
    }
    next_string
}

#[test]
fn test_case_2() {
    let code = "379A";
    let correct = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
    let actual = through_all_keypads(code);
    println!("actual : {}", actual);
    println!("correct: {}", correct);
    //assert!(actual == correct);
}

fn load_data(path: &str) -> Vec<String> {
    std::fs::read_to_string(path).unwrap().lines().map(|s| s.to_string()).collect()
}

#[named]
fn part1() {
    use real_data as data;
    let codes = load_data(data::FILENAME);
    let final_sum = codes.iter().map(|code| 
        {
            let result = through_all_keypads(code);
            let num: i32 = code[0..code.len() - 1].parse().unwrap();
            let len = result.len() as i32;
            let complexity = num * len;
            println!("{}: {}, {}*{} = {}", code, result, num, len, complexity);
            complexity
        }).sum::<i32>();
    println!("{}: {:?}", function_name!(), final_sum);
}

#[named]
fn part2() {
    use test_data as data;
    let codes = load_data(data::FILENAME);
    println!("{}: {:?}", function_name!(), codes);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 
// part2: 
