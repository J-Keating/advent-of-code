use std::{fmt, fs, io::BufRead};

use ::function_name::named;
use itertools::Itertools;
//use lazy_static::lazy_static;

use utilities::{alloc_2d_vec, Point};

#[derive(Copy, Clone)]
struct Node<T> {
    pub value: T,
    pub prev: usize,
    pub next: usize,
}

struct ArrayList<T> {
    pub data: Vec<Node<T>>,
}

impl<T> ArrayList<T> where T: std::fmt::Display + Copy,
{
    pub fn build(&mut self, in_data: &Vec<T>) {
        self.data = in_data
            .iter()
            .map(|&n| Node {
                value: n,
                prev: usize::MAX,
                next: usize::MAX,
            })
            .collect_vec();
        let len = self.data.len();
        self.data[0].prev = len - 1;
        self.data[0].next = 1;
        self.data[len - 1].prev = len - 2;
        self.data[len - 1].next = 0;
        for i in 1..len - 1 {
            self.data[i].prev = i - 1;
            self.data[i].next = i + 1;
        }
    }

    pub fn remove_node(&mut self, index_to_remove: usize) {
        // Close hole from where it was
        let old_prev = self.data[index_to_remove].prev;
        let old_next = self.data[index_to_remove].next;
        self.data[old_prev].next = old_next;
        self.data[old_next].prev = old_prev;
    }

    pub fn insert_node_after(&mut self, index_to_insert: usize, new_before_index: usize) {
        // Hook into new location
        let new_prev = new_before_index;
        let new_next = self.data[new_before_index].next;
        self.data[new_prev].next = index_to_insert;
        self.data[new_next].prev = index_to_insert;
        self.data[index_to_insert].prev = new_prev;
        self.data[index_to_insert].next = new_next;
    }

    pub fn move_to_after(&mut self, index_to_move: usize, new_before_index: usize) {
        self.remove_node(index_to_move);
        self.insert_node_after(index_to_move, new_before_index)
    }

    pub fn move_by(&mut self, index_to_move: usize, positions_to_move: i64) {
        self.remove_node(index_to_move);
        let mut new_prev = self.data[index_to_move].prev;
        if positions_to_move >= 0 {
            let steps = positions_to_move % (self.data.len() as i64 - 1);
            for _ in 0..steps {
                new_prev = self.data[new_prev].next;
            }
        }
        else {
            let steps = -positions_to_move % (self.data.len() as i64 - 1);
            for _ in 0..steps {
                new_prev = self.data[new_prev].prev;
            }
        }
        self.insert_node_after(index_to_move, new_prev);
    }

    pub fn print(&self) {
        let node_count = self.data.len();
        let mut loc = 0;
        for _ in 0..node_count+2 {
            print!("{},", self.data[loc].value);
            loc = self.data[loc].next;
        }
        print!("     ");
        loc = 0;
        for _ in 0..node_count+2 {
            print!("{},", self.data[loc].value);
            loc = self.data[loc].prev;
        }
        println!("");
    }
}

#[test]
fn test_arraylist() {
    {
        let data = vec![0, 1, 2, 3, 4, 5];
        let mut ar = ArrayList { data: Vec::new() };
        ar.build(&data);
        ar.print();
        ar.move_to_after(1, 2);
        ar.print();
        ar.move_to_after(3, 2);
        ar.print();
        ar.move_to_after(3, 4);
        ar.print();
        println!("");
    }
    {
        let data = vec![0, 1, 2, 3, 4, 5];
        let mut ar = ArrayList { data: Vec::new() };
        ar.build(&data);
        ar.print();
        ar.move_by(1, 1);
        ar.print();
        ar.move_by(4, 2);
        ar.print();
        ar.move_by(4, -2);
        ar.print();
        ar.move_by(1, -1);
        ar.print();
    }
}

fn load_data(path: &str, multiplier: i64) -> ArrayList<i64> {
    let file_contents = fs::read(path).expect("Error loading file");
    let data = file_contents.lines().map(|line| line.unwrap().parse::<i64>().unwrap() * multiplier).collect_vec();

    let mut ar = ArrayList { data: Vec::new() };
    ar.build(&data);
    ar
}

#[named]
fn part1() {
    let mut ar = load_data("src\\d20\\data.txt", 1);
    //ar.print();
    for i in 0..ar.data.len() {
        ar.move_by(i, ar.data[i].value);
    }
    //ar.print();
    let zero_pos_tuples = ar.data.iter().enumerate().filter(|(_, &v)| v.value == 0).collect_vec();
    assert!(zero_pos_tuples.len() == 1);
    let mut pos = zero_pos_tuples[0].0;
    // sum numbers 1000, 2000, and 3000 after the 0
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            pos = ar.data[pos].next;
        }
        println!("{}", ar.data[pos].value);
        sum += ar.data[pos].value;
    }

    println!("{}: {}", function_name!(), sum);
}

#[named]
fn part2() {
    let mut ar = load_data("src\\d20\\data.txt", 811589153);
    //ar.print();
    for mix_count in 0..10 {
        println!("{}", mix_count);
        for i in 0..ar.data.len() {
            ar.move_by(i, ar.data[i].value);
        }
    }
    //ar.print();
    let zero_pos_tuples = ar.data.iter().enumerate().filter(|(_, &v)| v.value == 0).collect_vec();
    assert!(zero_pos_tuples.len() == 1);
    let mut pos = zero_pos_tuples[0].0;
    // sum numbers 1000, 2000, and 3000 after the 0
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            pos = ar.data[pos].next;
        }
        println!("{}", ar.data[pos].value);
        sum += ar.data[pos].value;
    }

    println!("{}: {}", function_name!(), sum);
}

pub fn run() {
    part1();
    part2();
}

// part1: 4267
// part2: 6871725358451