use function_name::named;
use itertools::Itertools;
use std::{collections::HashSet, fs, mem::swap};

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

#[allow(dead_code)]
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

struct ComputerGenerator {
    key_char: char,
    char_counter: i32,
}

// Generate all combinations of a key character with the other 25 characters
// eg:  ta, tb, tc ... tz, at, bt, ct, ... zt
impl Iterator for ComputerGenerator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        if self.char_counter < 26 {
            result = Some((self.key_char as i32 - 'a' as i32) * 26 + self.char_counter as i32);
            self.char_counter += 1;
        }
        result
    }
}

#[allow(dead_code)]
fn computer_generator(key_char: char) -> ComputerGenerator {
    ComputerGenerator {
        key_char,
        char_counter: 0,
    }
}

#[test]
fn test_generator() {
    let g = computer_generator('t');
    for c in g {
        println!("{}, {}", c, decode_computer(c));
    }
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
    use real_data as data;
    let edges = load_data(data::FILENAME);
    let mut computer_grid = vec![vec![]; 26 * 26];
    for (a, b) in edges {
        computer_grid[a as usize].push(b);
        computer_grid[b as usize].push(a);
    }
    //println!("{}: {}, {:?}", function_name!(), edges.len(), edges[0..2].to_vec());
    let mut unique_sets = HashSet::<Vec<i32>>::new();
    for c1 in computer_generator('t') {
        for c2 in computer_grid[c1 as usize].iter() {
            for c3 in computer_grid[*c2 as usize].iter() {
                for c4 in computer_grid[*c3 as usize].iter() {
                    if c4 == &c1 {
                        //println!("{}: {} -> {} -> {} -> {}", function_name!(), decode_computer(c1), decode_computer(*c2), decode_computer(*c3), decode_computer(*c4));
                        let (mut s1, mut s2, mut s3) = (c1, *c2, *c3);
                        if s1 > s2 { swap(&mut s1, &mut s2);}
                        if s2 > s3 { swap(&mut s2, &mut s3);}
                        if s1 > s2 { swap(&mut s1, &mut s2);}
                        unique_sets.insert(vec![s1, s2, s3]);
                    }
                }
            }
        }
    }
    // for s in &unique_sets {
    //     println!("{}: {} -> {} -> {}", function_name!(), decode_computer(s[0]), decode_computer(s[1]), decode_computer(s[2]));
    // }
    println!("{}: {}", function_name!(), unique_sets.len());
}

fn full_connectivity(computer_grid: &Vec<Vec<i32>>, computers: &Vec<&i32>) -> bool {
    for c in computers {
        let connected_computers = &computer_grid[**c as usize];
        if !computers.iter().all(|c2| connected_computers.contains(c2)) {
            return false;
        }
    }
    true
}

#[named]
fn part2() {
    use real_data as data;
    let edges = load_data(data::FILENAME);
    let mut computer_grid = vec![vec![]; 26 * 26];
    for (a, b) in edges {
        computer_grid[a as usize].push(b);
        computer_grid[b as usize].push(a);
    }
    for (i, c) in computer_grid.iter_mut().enumerate() {
        c.push(i as i32);
        c.sort();
    }
    let mut size_to_test = computer_grid.iter().map(|c| c.len()).max().unwrap();
    let mut largest_set: Vec<i32> = vec![];
    while largest_set.is_empty() && size_to_test > 0 {
        let mut candidates = HashSet::new();
        for c in computer_grid.iter() {
            if c.len() == size_to_test {
                candidates.insert(c.clone());
            }
            else if c.len() > size_to_test {
                c.iter().combinations(size_to_test).for_each(|p| {
                    if full_connectivity(&computer_grid, &p) {
                        largest_set = p.iter().map(|c| **c).collect();
                    }
                });
            }
            if !largest_set.is_empty() {
                break;
            }
        }
        size_to_test -= 1;
    }
    println!("{}: {}", function_name!(), largest_set.iter().map(|c| decode_computer(*c)).collect::<Vec<_>>().join(","));
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 1308
// part2: bu,fq,fz,pn,rr,st,sv,tr,un,uy,zf,zi,zy
