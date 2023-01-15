use std::{fs, collections::VecDeque, time::Instant};

use ::function_name::named;
use utilities::{alloc_2d_vec, valid_vec_index};
//use itertools::Itertools;
//use lazy_static::lazy_static;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    //pub level: usize,
    pub r: i32,
    pub c: i32
}

fn process_neighbor(
    grid: &Vec<Vec<char>>,
    best_distances: &mut Vec<Vec<i32>>,
    loc: Point,
    from_height: char,
    mut distance: i32,
    could_be_start: Option<fn(char) -> bool>,
    to_process: &mut VecDeque<Point>
){
    if !valid_vec_index(&grid, loc.r) || !valid_vec_index(&grid[0], loc.c) {
        return;
    }
    let to_height = grid[loc.r as usize][loc.c as usize];
    if to_height as i32 - from_height as i32 > 1 {
        return;
    }
    if could_be_start.is_some() && could_be_start.unwrap()(to_height) {
        distance = 0;
    }
    if distance >= best_distances[loc.r as usize][loc.c as usize] {
        return;
    }
    best_distances[loc.r as usize][loc.c as usize] = distance;
    to_process.push_back(loc);
}

fn find_shortest_path(valid_start_loc_test: Option<fn(char) -> bool>) -> i32 {
    // Read grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    let file_contents = fs::read_to_string("src\\d12\\data.txt").expect("Error loading file");
    for line in file_contents.split("\r\n") {
        grid.push(line.chars().map(|c| c).collect());
    }
    
    // Find start and end points
    let mut start = Point { r: 0, c:0 };
    let mut end = Point { r: 0, c:0 };
    let (height, width) = (grid.len(), grid[0].len());

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, val) in row.iter().enumerate() {
            match val {
                'S' => { start = Point { r: row_index as i32, c: col_index as i32 }; }
                'E' => { end = Point { r: row_index as i32, c: col_index as i32  }; }
                _ => {}
            }    
        }    
    }
    // Set height values in start/end
    grid[start.r as usize][start.c as usize] = 'a';
    grid[end.r as usize][end.c as usize] = 'z';

    // Grid for shortest distance to each point
    let mut best_distances = alloc_2d_vec(height, width, (height * width) as i32);
    best_distances[start.r as usize][start.c as usize] = 0;

    // Start with the start point
    let mut to_process: VecDeque<Point> = VecDeque::new();
    to_process.push_back(start);
    
    // Do the work
    while !to_process.is_empty() {
        let p = to_process.pop_front().unwrap();
        let height = grid[p.r as usize][p.c as usize];
        let dist = best_distances[p.r as usize][p.c as usize];
        process_neighbor(&grid, &mut best_distances, Point { r: p.r - 1, c: p.c }, height, dist + 1, valid_start_loc_test, &mut to_process);
        process_neighbor(&grid, &mut best_distances, Point { r: p.r + 1, c: p.c }, height, dist + 1, valid_start_loc_test, &mut to_process);
        process_neighbor(&grid, &mut best_distances, Point { r: p.r, c: p.c - 1 }, height, dist + 1, valid_start_loc_test, &mut to_process);
        process_neighbor(&grid, &mut best_distances, Point { r: p.r, c: p.c + 1 }, height, dist + 1, valid_start_loc_test, &mut to_process);
    }

    best_distances[end.r as usize][end.c as usize]
}

#[named]
fn part1() {
    let now = Instant::now();

    // There is no other potential start location beyond the initial 'S'
    let valid_start_loc_test: Option<fn(char) -> bool> = None;
    let shortest_path = find_shortest_path(valid_start_loc_test);
    println!("{}: {} ({} ms)", function_name!(), shortest_path, now.elapsed().as_micros() as f32 / 1000.0);
}

#[named]
fn part2() {
    let now = Instant::now();

    // Any 'a' in the grid can be a valid start location
    let valid_start_loc_test: Option<fn(char) -> bool> = Some(|c| c == 'a');
    let shortest_path = find_shortest_path(valid_start_loc_test);
    println!("{}: {} ({} ms)", function_name!(), shortest_path, now.elapsed().as_micros() as f32 / 1000.0);
}

pub fn run() {
    part1();
    part2();
}

// part1: 
// part2: 